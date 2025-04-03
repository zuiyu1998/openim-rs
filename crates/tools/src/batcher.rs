use std::{
    collections::HashMap,
    error::Error,
    hash::{DefaultHasher, Hasher},
    mem::swap,
    time::Duration,
};

use tokio::{
    self,
    sync::mpsc::{self, Receiver, Sender},
    task::JoinSet,
    time::{interval, Interval},
};

use serde::{Deserialize, Serialize};

use async_trait::async_trait;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatcherConfig {
    max_count: usize,
    duration_ms: u64,
    buffer_size: usize,
    worker_count: usize,
}

pub trait BatcherData: 'static + Send + Sync {
    fn key(&self) -> String;
}

pub struct PayloadData<Data> {
    payload: Vec<Data>,
    pub key: String,
}

impl<Data> PayloadData<Data> {
    pub fn get_data(&self) -> &Vec<Data> {
        &self.payload
    }
}

#[derive(Debug, Clone)]
pub struct Batcher<Data>
where
    Data: BatcherData,
{
    data_sender: Option<Sender<Data>>,
    config: BatcherConfig,
}

impl<Data> Batcher<Data>
where
    Data: BatcherData,
{
    pub fn new(config: &BatcherConfig) -> Self {
        Self {
            data_sender: None,
            config: config.clone(),
        }
    }

    pub async fn put(&mut self, data: Data) {
        if let Some(sender) = self.data_sender.as_mut() {
            sender.send(data).await.unwrap();
        }
    }

    pub async fn start<Handler: BatcherHandler<Data = Data>>(&mut self, handler: Handler) {
        let (data_sender, data_receiver) = mpsc::channel(self.config.buffer_size);
        self.data_sender = Some(data_sender);

        let mut scheduler = Scheduler::new(
            handler,
            self.config.max_count,
            self.config.duration_ms,
            data_receiver,
            &self.config,
        );

        tokio::spawn(async move {
            scheduler.start().await;
        });
    }
}
pub struct Scheduler<Data: BatcherData> {
    max_count: usize,
    ticker: Interval,
    data_receiver: Receiver<Data>,
    values: HashMap<String, PayloadData<Data>>,
    count: usize,
    workers: Vec<Sender<(String, PayloadData<Data>)>>,
    join_set: JoinSet<()>,
    worker_count: usize,
}

impl<Data: BatcherData> Drop for Scheduler<Data> {
    fn drop(&mut self) {
        self.join_set.abort_all();
    }
}

impl<Data: BatcherData> Scheduler<Data> {
    pub fn new<Handler: BatcherHandler<Data = Data>>(
        handler: Handler,
        max_count: usize,
        duration_ms: u64,
        data_receiver: Receiver<Data>,
        config: &BatcherConfig,
    ) -> Self {
        let mut join_set = JoinSet::default();
        let mut workers: Vec<Sender<(String, PayloadData<Data>)>> = vec![];

        for _ in 0..config.worker_count {
            let (data_sender, mut data_receiver) = mpsc::channel(config.buffer_size);

            workers.push(data_sender);
            let handler_clone = handler.clone();

            join_set.spawn(async move {
                while let Some((key, payload)) = data_receiver.recv().await {
                    if let Err(e) = handler_clone.handle(key, payload).await {
                        tracing::error!("handler error: {}", e);
                    }
                }
            });
        }

        Scheduler {
            max_count,
            ticker: interval(Duration::from_millis(duration_ms)),
            data_receiver,
            values: Default::default(),
            count: 0,
            join_set,
            worker_count: config.worker_count,
            workers,
        }
    }

    fn reset(&mut self) {
        self.count = 0;
        self.ticker.reset();
    }

    pub async fn on_receiver_data(&mut self, data: Data) {
        self.count += 1;

        self.values
            .entry(data.key())
            .or_insert_with(|| PayloadData {
                key: data.key(),
                payload: vec![],
            })
            .payload
            .push(data);
    }

    pub async fn done(&mut self) {
        self.reset();

        let mut values: HashMap<String, PayloadData<Data>> = Default::default();

        swap(&mut values, &mut self.values);

        for (key, payload) in values.into_iter() {
            let index = share(&key) % (self.worker_count as u64);

            if let Err(e) = self.workers[index as usize].send((key, payload)).await {
                tracing::error!("send error: {}", e);
            }
        }
    }

    pub async fn start(&mut self) {
        loop {
            tokio::select! {
                optional_data = self.data_receiver.recv() => {
                    if let Some(data) = optional_data {
                        self.on_receiver_data(data).await;

                        if self.count > self.max_count {
                            self.done().await;
                        }

                    } else {
                        break;
                    }
                }
                _ = self.ticker.tick() => {
                    self.done().await;
                }
            }
        }
    }
}

pub fn share(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();

    hasher.write(key.as_bytes());
    hasher.finish()
}

#[async_trait]
pub trait BatcherHandler: Clone + Send + Sync + 'static {
    type Data: BatcherData;
    type Error: Error;

    async fn handle(
        &self,
        key: String,
        payload: PayloadData<Self::Data>,
    ) -> Result<(), Self::Error>;
}

mod test {

    use super::{BatcherData, BatcherHandler, PayloadData};
    use async_trait::async_trait;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum BatcherError {}

    #[derive(Debug)]
    pub struct TestBatcherData(String);

    impl BatcherData for TestBatcherData {
        fn key(&self) -> String {
            self.0.clone()
        }
    }

    #[derive(Debug, Clone)]
    pub struct TestBatcherHandler {}

    #[async_trait]
    impl BatcherHandler for TestBatcherHandler {
        type Data = TestBatcherData;
        type Error = BatcherError;

        async fn handle(
            &self,
            channel_id: String,
            payload: PayloadData<Self::Data>,
        ) -> Result<(), BatcherError> {
            for data in payload.payload.into_iter() {
                println!("key: {}, data: {:?}", channel_id, data);
            }

            Ok(())
        }
    }

    #[tokio::test]
    async fn test_batcher() {
        use super::{Batcher, BatcherConfig};
        use std::time::Duration;
        use tokio::time::sleep;

        let config = BatcherConfig {
            max_count: 10,
            duration_ms: 500,
            buffer_size: 100,
            worker_count: 10,
        };

        let mut batcher = Batcher {
            data_sender: None,
            config,
        };

        batcher.start(TestBatcherHandler {}).await;

        for i in 0..100 {
            batcher.put(TestBatcherData(format!("data: {}", i))).await;
        }

        sleep(Duration::from_secs(3)).await;
    }
}

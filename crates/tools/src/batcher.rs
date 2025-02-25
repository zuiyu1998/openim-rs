use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hasher},
    mem::swap,
    time::Duration,
};

use thiserror::Error;
use tokio::{
    self,
    sync::mpsc::{self, Receiver, Sender},
    time::{interval, Interval},
};

use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct BatcherConfig {
    max_count: usize,
    duration_ms: u64,
    worker: usize,
    buffer_size: usize,
}

#[derive(Debug, Error)]
pub enum BatcherError {}

pub trait BatcherData: 'static + Send + Sync {
    fn key(&self) -> String;
}

pub struct PayloadData<Data> {
    payload: Vec<Data>,
}

impl<Data> Default for PayloadData<Data> {
    fn default() -> Self {
        PayloadData { payload: vec![] }
    }
}

pub struct Batcher<Handler: BatcherHandler<Data = Data>, Data: BatcherData> {
    data_sender: Option<Sender<Data>>,
    handler: Handler,
    config: BatcherConfig,
}

impl<Handler: BatcherHandler<Data = Data>, Data: BatcherData> Batcher<Handler, Data> {
    pub async fn put(&mut self, data: Data) {
        if let Some(sender) = self.data_sender.as_mut() {
            sender.send(data).await.unwrap();
        }
    }

    pub async fn start(&mut self) {
        let (data_sender, data_receiver) = mpsc::channel(self.config.buffer_size);
        self.data_sender = Some(data_sender);

        let mut scheduler = Scheduler::new(
            self.handler.clone(),
            self.config.max_count,
            self.config.duration_ms,
            data_receiver,
            self.config.worker,
        );

        tokio::spawn(async move {
            scheduler.start().await;
        });
    }
}
pub struct Scheduler<Handler: BatcherHandler<Data = Data>, Data: BatcherData> {
    handler: Handler,
    max_count: usize,
    ticker: Interval,
    data_receiver: Receiver<Data>,
    values: HashMap<usize, PayloadData<Data>>,
    count: usize,
    worker: usize,
}

fn get_new_values<Data>(worker: usize) -> HashMap<usize, PayloadData<Data>> {
    let mut values: HashMap<usize, PayloadData<Data>> = HashMap::default();

    for id in 0..worker {
        values.insert(id, PayloadData::default());
    }

    values
}

fn get_hash(key: &str, worker: usize) -> usize {
    let mut hasher = DefaultHasher::default();
    hasher.write(key.as_bytes());
    let value = hasher.finish();

    (value % worker as u64) as usize
}

impl<Handler: BatcherHandler<Data = Data>, Data: BatcherData> Scheduler<Handler, Data> {
    pub fn new(
        handler: Handler,
        max_count: usize,
        duration_ms: u64,
        data_receiver: Receiver<Data>,
        worker: usize,
    ) -> Self {
        Scheduler {
            handler,
            max_count,
            ticker: interval(Duration::from_millis(duration_ms)),
            data_receiver,
            values: get_new_values(worker),
            count: 0,
            worker,
        }
    }

    fn reset(&mut self) {
        self.count = 0;
        self.ticker.reset();
    }

    pub async fn on_receiver_data(&mut self, data: Data) {
        self.count += 1;

        let index = get_hash(&data.key(), self.worker);

        self.values
            .entry(index)
            .or_insert(PayloadData::default())
            .payload
            .push(data);
    }

    pub async fn done(&mut self) {
        self.reset();

        let mut values = get_new_values::<Data>(self.worker);

        swap(&mut values, &mut self.values);

        for (id, payload) in values.into_iter() {
            let handler_clone = self.handler.clone();

            tokio::spawn(async move {
                if let Err(e) = handler_clone.handle(id, payload).await {
                    tracing::error!("handler error: {}", e);
                }
            });
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

#[async_trait]
pub trait BatcherHandler: Clone + Send + Sync + 'static {
    type Data: BatcherData;

    async fn handle(
        &self,
        channel_id: usize,
        payload: PayloadData<Self::Data>,
    ) -> Result<(), BatcherError>;
}

mod test {

    use super::{BatcherData, BatcherError, BatcherHandler, PayloadData};
    use async_trait::async_trait;

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

        async fn handle(
            &self,
            channel_id: usize,
            payload: PayloadData<Self::Data>,
        ) -> Result<(), BatcherError> {
            for data in payload.payload.into_iter() {
                println!("channel_id: {}, data: {:?}", channel_id, data);
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
            worker: 10,
        };

        let mut batcher = Batcher {
            data_sender: None,
            handler: TestBatcherHandler {},
            config,
        };

        batcher.start().await;

        for i in 0..100 {
            batcher.put(TestBatcherData(format!("data: {}", i))).await;
        }

        sleep(Duration::from_secs(3)).await;
    }
}

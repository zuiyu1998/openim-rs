use std::{collections::HashMap, error::Error, mem::swap, time::Duration};

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
    buffer_size: usize,
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

pub struct Batcher<Data, E, Handler>
where
    Data: BatcherData,
    E: Error,
    Handler: BatcherHandler<Data = Data, Error = E>,
{
    data_sender: Option<Sender<Data>>,
    handler: Handler,
    config: BatcherConfig,
}

impl<Data, Handler, E> Batcher<Data, E, Handler>
where
    Data: BatcherData,
    E: Error,
    Handler: BatcherHandler<Data = Data, Error = E>,
{
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
    values: HashMap<String, PayloadData<Data>>,
    count: usize,
}

impl<Handler: BatcherHandler<Data = Data>, Data: BatcherData> Scheduler<Handler, Data> {
    pub fn new(
        handler: Handler,
        max_count: usize,
        duration_ms: u64,
        data_receiver: Receiver<Data>,
    ) -> Self {
        Scheduler {
            handler,
            max_count,
            ticker: interval(Duration::from_millis(duration_ms)),
            data_receiver,
            values: Default::default(),
            count: 0,
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
            let handler_clone = self.handler.clone();

            tokio::spawn(async move {
                if let Err(e) = handler_clone.handle(key, payload).await {
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

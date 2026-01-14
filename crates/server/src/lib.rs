use rtrb::RingBuffer;
use std::thread;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::LocalRuntime;

pub struct Server {
    listener: TcpListener,
    num_workers: usize,
}

pub struct ServerConfig {
    pub addr: String,
    pub max_workers: Option<usize>,
}

impl Server {
    pub async fn new(config: ServerConfig) -> Result<Self, std::io::Error> {
        let num_workers = config.max_workers.unwrap_or(1);
        let listener = TcpListener::bind(&config.addr).await?;
        Ok(Server {
            listener,
            num_workers,
        })
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers: Vec<rtrb::Producer<TcpStream>> = Vec::with_capacity(self.num_workers);
        for cpu_id in 0..self.num_workers {
            let (producer, mut consumer) = RingBuffer::new(1024);
            workers.push(producer);

            thread::spawn(move || {
                // pin to core!
                let core_ids = core_affinity::get_core_ids().unwrap();
                if let Some(core_id) = core_ids.get(cpu_id) {
                    core_affinity::set_for_current(*core_id);
                }

                let rt = LocalRuntime::new().expect("Failed to create Tokio LocalRuntime");

                rt.block_on(async move {
                    loop {
                        if let Ok(mut stream) = consumer.pop() {
                            tokio::spawn(async move {
                                // todo - implement blockstore.
                                let _ = stream.write_all(b"my_value").await;
                            });
                        }

                        tokio::task::yield_now().await; // todo - why this?
                    }
                });
            });
        }

        println!("Listening on {}", self.listener.local_addr()?);
        loop {
            let (stream, _) = self.listener.accept().await?;
            let worker_id = 0;
            workers[worker_id].push(stream).unwrap(); // todo - handle failures
        }
    }
}

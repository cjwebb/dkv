mod error;

use protocol::Response;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub type Result<T> = std::result::Result<T, error::ClientError>;

pub struct Client {
    stream: TcpStream,
}

// todo - delegate key/value to protocol module, and use proper serialization
impl Client {
    pub async fn connect(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Client { stream })
    }

    pub async fn set(&mut self, key: &str, value: &str) -> Result<Response> {
        self.stream
            .write_all(format!("SET {} {}", key, value).as_bytes())
            .await?;
        self.stream.flush().await?;
        Ok(Response::Ok {
            value: Option::None, // todo - make this nicer
        })
    }

    pub async fn get(&mut self, key: &str) -> Result<Response> {
        let mut buff = vec![0; 1024];
        let n = self.stream.read(&mut buff).await?;
        buff.truncate(n);
        Ok(Response::Ok {
            value: Option::Some(buff),
        })
    }
}

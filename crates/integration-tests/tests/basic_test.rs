use client::Client;
use server::{Server, ServerConfig};

#[tokio::test]
async fn test_basic_get_set() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "localhost:12000";

    let _server = {
        let s = Server::new(ServerConfig {
            addr: server_addr.to_string(),
            max_workers: Some(1),
        })
        .await?;

        tokio::spawn(async move {
            s.start().await.unwrap();
        });
    };

    let mut client = Client::connect(server_addr).await?;

    let key = "my_key";
    let value = "my_value";

    client.set(key, value).await?;
    let response = client.get(key).await?;

    match response {
        protocol::Response::Ok { value: Some(v) } => {
            assert_eq!(v, value.as_bytes());
        }
        _ => panic!("Unexpected response"),
    }

    Ok(())
}

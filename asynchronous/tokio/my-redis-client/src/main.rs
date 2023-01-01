
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let mut client = client::connect("127.0.0.1:6667").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("Value = {:?}", result);

    Ok(())
}

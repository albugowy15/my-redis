use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // open connection to mini redis server
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set a new key-value pair
    client.set("hello", "word".into()).await?;

    // try get the value with key "hello"
    let result = client.get("hello").await?;

    // print the value
    println!("got value from server; result {:?}", result);

    Ok(())
}

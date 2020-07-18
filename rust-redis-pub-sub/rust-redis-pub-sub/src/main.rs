use futures_util::StreamExt as _;
use redis::AsyncCommands;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://redis-broker/").unwrap();
    let mut publish_conn = client.get_async_connection().await?;
    let mut pubsub_conn = client.get_async_connection().await?.into_pubsub();

    pubsub_conn.subscribe("hello").await?;
    let mut pubsub_stream = pubsub_conn.on_message();

    publish_conn.publish("hello", "world").await?;

    let pubsub_msg: String = pubsub_stream.next().await.unwrap().get_payload()?;
    println!("Recieved message with payload: {}", &pubsub_msg);
    assert_eq!(&pubsub_msg, "world");

    Ok(())
}
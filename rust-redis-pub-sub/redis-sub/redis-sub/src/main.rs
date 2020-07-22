fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://redis-broker/")?;
    let mut con = client.get_connection()?;
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("frodo")?;
    pubsub.subscribe("tom")?;

    loop {
        let msg = pubsub.get_message()?;
        let payload : String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
}

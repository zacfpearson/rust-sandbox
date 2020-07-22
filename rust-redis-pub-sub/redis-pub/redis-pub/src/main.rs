use redis::Commands;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://redis-broker/")?;
    let mut con = client.get_connection()?;

    for _ in 1..10 {  
        con.publish("frodo","baggins")?;
        std::thread::sleep(std::time::Duration::from_millis(1000));

        con.publish("tom","bombadil")?;
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    Ok(())
}
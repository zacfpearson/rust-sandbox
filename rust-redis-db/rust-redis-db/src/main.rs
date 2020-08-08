use redis::Commands;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://redis-broker/")?;
    let mut con = client.get_connection()?;

    let (character1_lastname, character2_lastname) : (String, String) = redis::pipe()
        .atomic()
        .set("frodo", "bagins").ignore()
        .set("Rand", "al'thor").ignore()
        .get("frodo")
        .get("Rand").query(&mut con)?;

    println!("Character 1 last name: {}",character1_lastname);
    println!("Character 2 last name: {}",character2_lastname);

    Ok(())
}

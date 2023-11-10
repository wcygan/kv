use client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("http://localhost:7676".to_string()).await?;

    client.put("foo".to_string(), "bar".to_string()).await?;

    let value = client.get::<String, String>("foo".to_string()).await?;

    println!("value: {:?}", value);

    Ok(())
}

use futures::stream::iter;
use hello::say_client::SayClient;
use hello::SayRequest;
mod hello;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://grpc-server:50051")
        .connect()
        .await?;
    let mut client = SayClient::new(channel);
// creating a stream
    let request = tonic::Request::new(iter(vec![
        SayRequest {
            name: String::from("zac"),
        },
        SayRequest {
            name: String::from("franklin"),
        },
        SayRequest {
            name: String::from("pearson"),
        },
    ]));
// sending stream
    let response = client.receive_stream(request).await?.into_inner();
    println!("RESPONSE=\n{}", response.message);
    Ok(())
}


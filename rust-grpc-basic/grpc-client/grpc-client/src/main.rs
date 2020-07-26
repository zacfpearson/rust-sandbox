use hello::say_client::SayClient;
use hello::SayRequest;

mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://grpc-server:50051")
    .connect()
    .await?;
// creating gRPC client from channel
    let mut client = SayClient::new(channel);
// creating a new Request
    let request = tonic::Request::new(
        SayRequest {
           name:String::from("Zac")
        },
    );
// sending request and waiting for response
    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}

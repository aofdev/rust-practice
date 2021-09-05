use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creating a channel ie connection to server
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // Creating a new Request
    let request = tonic::Request::new(HelloRequest {
        name: "Hello Tonic".into(),
    });

    // Sending request and waiting for response
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}

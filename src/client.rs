use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloRequest};
use std::time::{Instant};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let now = Instant::now();

    for i in 1..10000 {

        let mut client_closure = client.clone();

        tokio::spawn(async move {
            let request = tonic::Request::new(HelloRequest {
                name: i32::to_string(&i).into(),
            });

            client_closure.say_hello(request).await
        });
    }

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);

    let elapsed = now.elapsed();
    println!("Time elapsed: {} ms, {} s", elapsed.as_micros(), elapsed.as_secs_f32());

    Ok(())
}
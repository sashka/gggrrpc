use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

use num_cpus;

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        // println!("Got a request from {:?}", request.remote_addr());
        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
            num: num_cpus::get() as u32,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let cert = tokio::fs::read("minica.pem").await?;
    // let key = tokio::fs::read("minica-key.pem").await?;
    // let identity = Identity::from_pem(cert, key);

    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        // .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

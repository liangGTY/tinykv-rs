mod config;
mod storage;
mod server;

use clap::Parser;
use storage::standalone_storage::{StandaloneStorage};
use tonic::{transport::Server, Request, Response, Status};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloReply};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    scheduler: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    addr: u8,
    path: String,
    loglevel: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut config = config::Config::default();

    if args.path != "" {
        config.dbpath = args.path;
    }

    let storage = Box::new(StandaloneStorage::new(config));
    let server = server::Server::new(storage);
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    tonic::transport::Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
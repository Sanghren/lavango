use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("vm.proto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    Server::builder()
        .add_service()
        .serve(addr)
        .await?;
    Ok(())
}

use rpcchainvm::VersionRequest;
use rpcchainvm::vm_client::VmClient;

pub mod rpcchainvm {
    tonic::include_proto!("vmproto");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = VmClient::connect("http://127.0.0.1:10002").await?;

    let request = tonic::Request::new(VersionRequest {});

    let response = client.version(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

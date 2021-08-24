use tonic::{transport::Server, Request, Response, Status};
use rpcchainvm::vm_server::{VmServer};

pub mod rpcchainvm {
    tonic::include_proto!("vmproto");
}

#[derive(Debug, Default)]
pub struct RpcChainVm {}

#[tonic::async_trait]
impl rpcchainvm::vm_server::Vm for RpcChainVm {
    #[tracing::instrument]
    async fn initialize(
        &self,
        request: tonic::Request<rpcchainvm::InitializeRequest>,
    ) -> Result<tonic::Response<rpcchainvm::InitializeResponse>, tonic::Status> {
        Ok(Response::new(rpcchainvm::InitializeResponse {
            last_accepted_id: vec![],
            last_accepted_parent_id: vec![],
            status: 0,
            height: 1,
            bytes: vec![],
        }))
    }
    #[tracing::instrument]
    async fn bootstrapping(
        &self,
        request: tonic::Request<rpcchainvm::BootstrappingRequest>,
    ) -> Result<tonic::Response<rpcchainvm::BootstrappingResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::BootstrappingResponse{}))
    }
    #[tracing::instrument]
    async fn bootstrapped(
        &self,
        request: tonic::Request<rpcchainvm::BootstrappedRequest>,
    ) -> Result<tonic::Response<rpcchainvm::BootstrappedResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::BootstrappedResponse{}))
    }
    #[tracing::instrument]
    async fn shutdown(
        &self,
        request: tonic::Request<rpcchainvm::ShutdownRequest>,
    ) -> Result<tonic::Response<rpcchainvm::ShutdownResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::ShutdownResponse{}))
    }
    #[tracing::instrument]
    async fn create_handlers(
        &self,
        request: tonic::Request<rpcchainvm::CreateHandlersRequest>,
    ) -> Result<tonic::Response<rpcchainvm::CreateHandlersResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::CreateHandlersResponse{
            handlers: vec![]
        }))
    }
    #[tracing::instrument]
    async fn create_static_handlers(
        &self,
        request: tonic::Request<rpcchainvm::CreateStaticHandlersRequest>,
    ) -> Result<tonic::Response<rpcchainvm::CreateStaticHandlersResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::CreateStaticHandlersResponse{
            handlers: vec![]
        }))
    }
    #[tracing::instrument]
    async fn build_block(
        &self,
        request: tonic::Request<rpcchainvm::BuildBlockRequest>,
    ) -> Result<tonic::Response<rpcchainvm::BuildBlockResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::BuildBlockResponse{
            id:  vec![],
            parent_id: vec![],
            bytes: vec![],
            height: 1,
        }))
    }
    #[tracing::instrument]
    async fn parse_block(
        &self,
        request: tonic::Request<rpcchainvm::ParseBlockRequest>,
    ) -> Result<tonic::Response<rpcchainvm::ParseBlockResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::ParseBlockResponse{
            id: vec![],
            parent_id: vec![],
            status: 1,
            height: 2
        }))
    }
    #[tracing::instrument]
    async fn get_block(
        &self,
        request: tonic::Request<rpcchainvm::GetBlockRequest>,
    ) -> Result<tonic::Response<rpcchainvm::GetBlockResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::GetBlockResponse{
            parent_id: vec![],
            bytes: vec![],
            status: 1,
            height: 2,
        }))
    }
    #[tracing::instrument]
    async fn set_preference(
        &self,
        request: tonic::Request<rpcchainvm::SetPreferenceRequest>,
    ) -> Result<tonic::Response<rpcchainvm::SetPreferenceResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::SetPreferenceResponse{}))
    }

    #[tracing::instrument]
    async fn version(
        &self,
        request: tonic::Request<rpcchainvm::VersionRequest>,
    ) -> Result<tonic::Response<rpcchainvm::VersionResponse>, tonic::Status>{

        tracing::info!("received request");

        Ok(Response::new(rpcchainvm::VersionResponse{
            version: "1".to_string()
        }))
    }
    #[tracing::instrument]
    async fn block_verify(
        &self,
        request: tonic::Request<rpcchainvm::BlockVerifyRequest>,
    ) -> Result<tonic::Response<rpcchainvm::BlockVerifyResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::BlockVerifyResponse{}))
    }
    #[tracing::instrument]
    async fn block_accept(
        &self,
        request: tonic::Request<rpcchainvm::BlockAcceptRequest>,
    ) -> Result<tonic::Response<rpcchainvm::BlockAcceptResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::BlockAcceptResponse{}))
    }
    #[tracing::instrument]
    async fn block_reject(
        &self,
        request: tonic::Request<rpcchainvm::BlockRejectRequest>,
    ) -> Result<tonic::Response<rpcchainvm::BlockRejectResponse>, tonic::Status>{
        Ok(Response::new(rpcchainvm::BlockRejectResponse{}))
    }
    #[tracing::instrument]
    async fn health(
        &self,
        request: tonic::Request<rpcchainvm::HealthRequest>,
    ) -> Result<tonic::Response<rpcchainvm::HealthResponse>, tonic::Status> {
        tracing::info!("received request");

        Ok(Response::new(rpcchainvm::HealthResponse{
            details: "Bouh".to_string()
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // Needed for go-plugin
    // ToDo Look for each field and explain them
    println!("1|5|tcp|127.0.0.1:10002|grpc");

    let addr = "127.0.0.1:10002".parse().unwrap();
    let greeter = RpcChainVm::default();

    let server = VmServer::new(greeter);
    Server::builder()
        .add_service(server)
        .serve(addr)
        .await?;

    Ok(())
}

use cgp::prelude::*;
use tendermint_rpc::Url;
use tonic::client::Grpc;
use tonic::transport::Endpoint;

#[cgp_component {
    provider: GrpcAddressGetter,
    context: Chain,
}]
pub trait HasGrpcAddress: Async {
    fn grpc_address(&self) -> &Url;
}

#[async_trait]
pub trait CanQueryGrpcServiceStatus: HasAsyncErrorType {
    async fn query_grpc_service_status_is_ready(&self) -> Result<bool, Self::Error>;
}

impl<Chain> CanQueryGrpcServiceStatus for Chain
where
    Chain: HasGrpcAddress + CanRaiseAsyncError<tonic::transport::Error>,
{
    async fn query_grpc_service_status_is_ready(&self) -> Result<bool, Chain::Error> {
        let endpoint: Endpoint = self
            .grpc_address()
            .to_string()
            .try_into()
            .map_err(Chain::raise_error)?;

        let channel = endpoint.connect().await.map_err(Chain::raise_error)?;

        let mut rpc_client = Grpc::new(channel);

        let ready = rpc_client.ready().await.is_ok();

        Ok(ready)
    }
}

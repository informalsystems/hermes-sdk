use core::marker::PhantomData;

use cgp::core::component::UseContext;
use hermes_prelude::*;
use tendermint_rpc::HttpClient;

#[cgp_component {
  provider: RpcClientGetter,
  context: Client,
}]
pub trait HasRpcClient: Async {
    fn rpc_client(&self) -> &HttpClient;
}

#[cgp_provider(RpcClientGetterComponent)]
impl<Client> RpcClientGetter<Client> for UseContext
where
    Client: Async + HasField<symbol!("rpc_client"), Value = HttpClient>,
{
    fn rpc_client(client: &Client) -> &HttpClient {
        client.get_field(PhantomData::<symbol!("rpc_client")>)
    }
}

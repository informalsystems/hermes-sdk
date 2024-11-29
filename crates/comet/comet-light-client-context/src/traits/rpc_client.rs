use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use tendermint_rpc::HttpClient;

#[derive_component(RpcClientGetterComponent, RpcClientGetter<Client>)]
pub trait HasRpcClient: Async {
    fn rpc_client(&self) -> &HttpClient;
}

impl<Client> RpcClientGetter<Client> for UseContext
where
    Client: Async + HasField<symbol!("rpc_client"), Field = HttpClient>,
{
    fn rpc_client(client: &Client) -> &HttpClient {
        client.get_field(PhantomData::<symbol!("rpc_client")>)
    }
}

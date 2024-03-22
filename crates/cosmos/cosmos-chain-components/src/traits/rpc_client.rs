use cgp_core::prelude::*;
use tendermint_rpc::{HttpClient, Url};

#[derive_component(RpcClientGetterComponent, RpcClientGetter<Chain>)]
pub trait HasRpcClient: Async {
    fn rpc_client(&self) -> &HttpClient;

    fn rpc_address(&self) -> &Url;
}

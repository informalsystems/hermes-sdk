use cgp::prelude::*;
use tendermint_rpc::{HttpClient, Url};

#[cgp_component {
  provider: RpcClientGetter,
  context: Chain,
}]
pub trait HasRpcClient: Async {
    fn rpc_client(&self) -> &HttpClient;

    fn rpc_address(&self) -> &Url;
}

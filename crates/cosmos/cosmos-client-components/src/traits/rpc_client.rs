use cgp_core::Async;
use tendermint_rpc::{HttpClient, Url};

pub trait HasRpcClient: Async {
    fn rpc_client(&self) -> &HttpClient;

    fn rpc_address(&self) -> &Url;
}

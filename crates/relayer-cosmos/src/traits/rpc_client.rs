use cgp_core::Async;
use tendermint_rpc::HttpClient;

pub trait HasRpcClient: Async {
    fn rpc_client(&self) -> &HttpClient;
}

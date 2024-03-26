use cgp_core::Async;
use jsonrpsee::http_client::HttpClient;

use crate::traits::json_rpc_client::ProvideJsonRpcClientType;

pub struct ProvideJsonRpseeClient;

impl<Rollup> ProvideJsonRpcClientType<Rollup> for ProvideJsonRpseeClient
where
    Rollup: Async,
{
    type JsonRpcClient = HttpClient;
}

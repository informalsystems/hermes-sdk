use cgp_core::prelude::*;
pub use jsonrpsee::core::client::error::Error as JsonRpcClientError;
use jsonrpsee::core::client::ClientT;

#[derive_component(JsonRpcClientTypeComponent, ProvideJsonRpcClientType<Chain>)]
pub trait HasJsonRpcClientType: Async {
    type JsonRpcClient: ClientT + Async;
}

#[derive_component(JsonRpcClientGetterComponent, JsonRpcClientGetter<Chain>)]
pub trait HasJsonRpcClient: HasJsonRpcClientType {
    fn json_rpc_client(&self) -> &Self::JsonRpcClient;
}

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use jsonrpsee::core::client::ClientT;

pub use jsonrpsee::core::client::error::Error as JsonRpcClientError;

#[derive_component(JsonRpcClientTypeComponent, ProvideJsonRpcClientType<Chain>)]
pub trait HasJsonRpcClientType: Async {
    type JsonRpcClient: ClientT + Async;
}

#[derive_component(JsonRpcClientGetterComponent, JsonRpcClientGetter<Chain>)]
pub trait HasJsonRpcClient: HasJsonRpcClientType + CanRaiseError<JsonRpcClientError> {
    fn json_rpc_client(&self) -> &Self::JsonRpcClient;
}

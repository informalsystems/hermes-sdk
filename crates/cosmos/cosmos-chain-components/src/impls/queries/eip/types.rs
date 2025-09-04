use ibc_proto::cosmos::base::v1beta1::DecCoin;

/// GasPriceRequest is the request type for the Query/GasPrice RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasPriceRequest {
    /// denom we are querying gas price in
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}

/// GasPriceResponse is the response type for the Query/GasPrice RPC method.
/// Returns a gas price in specified denom.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasPriceResponse {
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<DecCoin>,
}

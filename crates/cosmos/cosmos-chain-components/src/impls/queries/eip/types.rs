use ibc_proto::cosmos::base::v1beta1::DecCoin;
use reqwest::Response;
use serde::Deserialize;

#[derive(Debug)]
pub struct EipQueryError {
    pub response: Response,
}

#[derive(Deserialize)]
pub struct EipBaseFeeHTTPResult {
    pub result: EipBaseFeeResult,
}

#[derive(Deserialize)]
pub struct EipBaseFeeResult {
    pub response: EipBaseFeeResponse,
}

#[derive(Deserialize)]
pub struct EipBaseFeeResponse {
    pub value: String,
}

/// GasPriceResponse is the response type for the Query/GasPrice RPC method.
/// Returns a gas price in specified denom.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasPriceResponse {
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<DecCoin>,
}

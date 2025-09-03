use ibc_proto::cosmos::base::v1beta1::DecCoin;
use serde::Deserialize;
use tendermint_rpc::endpoint::abci_query::AbciQuery;

#[derive(Debug)]
pub struct EipQueryError {
    pub response: AbciQuery,
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

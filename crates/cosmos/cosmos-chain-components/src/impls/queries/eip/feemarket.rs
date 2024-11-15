use cgp::core::error::CanRaiseError;
use core::fmt;
use core::ops::Div;
use core::str::FromStr;
use prost::DecodeError;
use serde::Deserialize;
use subtle_encoding::base64;

use ibc_proto::cosmos::base::v1beta1::DecCoin;

use crate::impls::queries::eip::dispatch::EipQueryError;
use crate::traits::eip::eip_query::EipQuerier;
use crate::traits::rpc_client::HasRpcClient;

#[derive(Deserialize)]
struct EipBaseFeeHTTPResult {
    result: EipBaseFeeResult,
}

#[derive(Deserialize)]
struct EipBaseFeeResult {
    response: EipBaseFeeResponse,
}

#[derive(Deserialize)]
struct EipBaseFeeResponse {
    value: String,
}

/// GasPriceResponse is the response type for the Query/GasPrice RPC method.
/// Returns a gas price in specified denom.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasPriceResponse {
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<DecCoin>,
}

/// Extracted from `cosmwasm-std`
///
/// <https://docs.rs/cosmwasm-std/latest/src/cosmwasm_std/math/uint128.rs.html>
#[derive(Clone, Copy)]
struct Uint128(u128);

impl Uint128 {
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn checked_rem(&self, rhs: Self) -> Option<Self> {
        self.0.checked_rem(rhs.0).map(Self)
    }
}

impl FromStr for Uint128 {
    type Err = core::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u128>().map(Self)
    }
}

impl Div<Uint128> for Uint128 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(
            self.0
                .checked_div(rhs.0)
                .expect("attempt to divide by zero"),
        )
    }
}

impl fmt::Display for Uint128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Extracted from `cosmwasm-std`
///
/// <https://docs.rs/cosmwasm-std/latest/src/cosmwasm_std/math/decimal.rs.html>
#[derive(Clone, Copy)]
struct Decimal(Uint128);

impl Decimal {
    const DECIMAL_FRACTIONAL: Uint128 = Uint128::new(1_000_000_000_000_000_000u128); // 1*10**18
    pub const DECIMAL_PLACES: u32 = 18;

    pub const fn new(value: Uint128) -> Self {
        Self(value)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use core::fmt::Write;

        let whole = (self.0) / Self::DECIMAL_FRACTIONAL;
        let fractional = (self.0).checked_rem(Self::DECIMAL_FRACTIONAL).unwrap();

        if fractional.is_zero() {
            write!(f, "{whole}")
        } else {
            let fractional_string = format!(
                "{:0>padding$}",
                fractional,
                padding = Self::DECIMAL_PLACES as usize
            );
            f.write_str(&whole.to_string())?;
            f.write_char('.')?;
            f.write_str(fractional_string.trim_end_matches('0'))?;
            Ok(())
        }
    }
}

/// Query EIP-1559 base fee using Skip's feemarket endpoint and decode it using
/// `GasPriceResponse`
pub struct QueryEipFromFeeMarket;

impl<Chain> EipQuerier<Chain> for QueryEipFromFeeMarket
where
    Chain: HasRpcClient
        + CanRaiseError<reqwest::Error>
        + CanRaiseError<subtle_encoding::Error>
        + CanRaiseError<DecodeError>
        + CanRaiseError<core::num::ParseIntError>
        + CanRaiseError<core::num::ParseFloatError>
        + CanRaiseError<EipQueryError>,
{
    async fn query_eip_base_fee(chain: &Chain, denom: &str) -> Result<f64, Chain::Error> {
        let url = format!(
            "{}abci_query?path=\"/feemarket.feemarket.v1.Query/GasPrices\"&denom={denom}",
            chain.rpc_address()
        );

        let response = reqwest::get(&url).await.map_err(Chain::raise_error)?;

        if !response.status().is_success() {
            return Err(Chain::raise_error(EipQueryError { response }));
        }

        let result: EipBaseFeeHTTPResult = response.json().await.map_err(Chain::raise_error)?;

        let decoded = base64::decode(result.result.response.value).map_err(Chain::raise_error)?;

        let gas_price_response: GasPriceResponse =
            prost::Message::decode(decoded.as_ref()).map_err(Chain::raise_error)?;
        let dec_coin = gas_price_response.price.unwrap().clone();
        let base_fee_uint128 = Uint128::from_str(&dec_coin.amount).map_err(Chain::raise_error)?;

        let dec = Decimal::new(base_fee_uint128);
        let amount = f64::from_str(dec.to_string().as_str()).map_err(Chain::raise_error)?;

        Ok(amount)
    }
}

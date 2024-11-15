use core::fmt;
use core::ops::Div;
use core::str::FromStr;
use reqwest::Response;
use serde::Deserialize;

use ibc_proto::cosmos::base::v1beta1::DecCoin;

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

/// Extracted from `cosmwasm-std`
///
/// <https://docs.rs/cosmwasm-std/latest/src/cosmwasm_std/math/uint128.rs.html>
#[derive(Clone, Copy)]
pub struct Uint128(u128);

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
pub struct Decimal(Uint128);

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

use core::str::FromStr;

use eyre::Report;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub enum EipQueryType {
    #[default]
    FeeMarket,
    Osmosis,
}

impl FromStr for EipQueryType {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "feemarket" => Ok(Self::FeeMarket),
            "osmosis" => Ok(Self::Osmosis),
            _ => Err(Report::msg(format!("unknown EIP query type: {s}"))),
        }
    }
}

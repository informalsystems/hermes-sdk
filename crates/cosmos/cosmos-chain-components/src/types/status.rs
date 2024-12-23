use core::fmt::{Display, Formatter, Result};

use ibc::core::client::types::Height;
use serde::Serialize;
pub use tendermint::Time;

#[derive(Debug, Serialize)]
pub struct ChainStatus {
    pub height: Height,
    pub time: Time,
}

impl Display for ChainStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "height: {}, time: {}", self.height, self.time)
    }
}

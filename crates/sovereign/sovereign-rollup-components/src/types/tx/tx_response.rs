use serde::Deserialize;

use crate::types::event::SovereignEvent;
use crate::types::tx::tx_hash::TxHash;

#[derive(Debug, Deserialize)]
pub struct TxResponse {
    pub hash: TxHash,
    pub events: Vec<SovereignEvent>,
    pub custom_receipt: TxEffect,
}

#[derive(Debug, Deserialize)]
pub enum TxEffect {
    Reverted,
    Successful,
    CannotReserveGas,
    InsufficientBaseGas,
    Duplicate,
    CannotResolveContext,
}

pub enum TxError {
    Reverted,
    InsufficientBaseGas,
    Duplicate,
}

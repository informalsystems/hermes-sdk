use serde::Deserialize;

use crate::sovereign::types::event::SovereignEvent;
use crate::sovereign::types::rpc::tx_hash::TxHash;

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
    InsufficientBaseGas,
    Duplicate,
}

pub enum TxError {
    Reverted,
    InsufficientBaseGas,
    Duplicate,
}

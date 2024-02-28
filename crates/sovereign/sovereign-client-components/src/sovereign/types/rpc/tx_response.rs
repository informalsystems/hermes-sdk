use core::ops::Range;

use serde::Deserialize;

use crate::sovereign::types::event::RawEvent;
use crate::sovereign::types::rpc::tx_hash::TxHash;

#[derive(Debug, Deserialize)]
pub struct TxResponse {
    pub hash: TxHash,
    pub events: Vec<RawEvent>,
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

use borsh::BorshDeserialize;
use serde::Deserialize;

use crate::sovereign::types::events::bank::BankEvent;

#[derive(Debug, Deserialize)]
pub struct RawEvent {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Debug)]
pub struct SovereignEvent {
    pub key: String,
    pub detail: SovereignEventDetail,
}

#[derive(Debug, BorshDeserialize)]
pub enum SovereignEventDetail {
    Accounts,
    Bank(BankEvent),
}

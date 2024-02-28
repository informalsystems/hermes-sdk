use borsh::BorshDeserialize;

use crate::sovereign::types::events::bank::BankEvent;

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

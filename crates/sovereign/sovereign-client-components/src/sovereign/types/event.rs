use borsh::BorshDeserialize;
use serde::Deserialize;

use crate::sovereign::types::events::bank::BankEvent;

#[derive(Debug, Deserialize)]
pub struct SovereignEvent {
    pub event_value: serde_json::Value,
    pub module_name: String,
    pub module_address: String,
}

use borsh::BorshDeserialize;

use crate::sovereign::types::address::SovereignAddressBytes;

#[derive(Debug, BorshDeserialize)]
pub enum BankEvent {
    TokenCreated {
        token_address: SovereignAddressBytes,
    },
    TokenTransferred {
        token_address: SovereignAddressBytes,
    },
}

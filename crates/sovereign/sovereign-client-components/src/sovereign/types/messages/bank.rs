use borsh::BorshSerialize;

use crate::sovereign::types::address::SovereignAddressBytes;

#[derive(BorshSerialize)]
pub enum BankMessage {
    Transfer {
        to: SovereignAddressBytes,
        coins: CoinFields,
    },
}

#[derive(BorshSerialize)]
pub struct CoinFields {
    pub amount: u64,
    pub token_address: SovereignAddressBytes,
}

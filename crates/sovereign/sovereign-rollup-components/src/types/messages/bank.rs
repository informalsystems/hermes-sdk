use borsh::BorshSerialize;

use crate::types::address::SovereignAddressBytes;

#[derive(Debug, BorshSerialize)]
pub enum BankMessage {
    CreateToken {
        salt: u64,
        token_name: String,
        initial_balance: u64,
        minter_address: SovereignAddressBytes,
        authorized_minters: Vec<SovereignAddressBytes>,
    },
    Transfer {
        to: SovereignAddressBytes,
        coins: CoinFields,
    },
}

#[derive(Debug, BorshSerialize)]
pub struct CoinFields {
    pub amount: u64,
    pub token_address: SovereignAddressBytes,
}

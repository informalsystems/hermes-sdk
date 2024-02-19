use borsh::BorshSerialize;

#[derive(BorshSerialize)]
pub enum BankMessage {
    Transfer { to: String, coins: CoinFields },
}

#[derive(BorshSerialize)]
pub struct CoinFields {
    pub amount: u64,
    pub token_address: String,
}

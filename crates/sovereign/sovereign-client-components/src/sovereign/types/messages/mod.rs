use serde::Serialize;

#[derive(Serialize)]
pub struct TransferMessage {
    pub to: String,
    pub coins: CoinMessage,
}

#[derive(Serialize)]
pub struct CoinMessage {
    pub amount: u64,
    pub token_address: String,
}

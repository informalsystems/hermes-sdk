use serde::Serialize;

#[derive(Serialize)]
pub enum TransferMessage {
    Transfer(TransferFields),
}

#[derive(Serialize)]
pub struct TransferFields {
    pub to: String,
    pub coins: CoinFields,
}

#[derive(Serialize)]
pub struct CoinFields {
    pub amount: u64,
    pub token_address: String,
}

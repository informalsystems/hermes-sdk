use crate::chain::types::Amount;

pub struct CosmosWalletConfig {
    pub wallet_id: String,
    pub genesis_balances: Vec<Amount>,
    pub validator_staked_amount: Option<Amount>,
}

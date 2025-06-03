#[derive(Clone, Debug)]
pub struct CosmosRecoverClientPayload {
    pub deposit_amount: u128,
    pub deposit_denom: String,
}

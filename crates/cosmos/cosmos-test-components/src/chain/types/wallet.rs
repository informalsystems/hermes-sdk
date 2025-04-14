use hermes_cosmos_chain_components::types::Secp256k1KeyPair;

#[derive(Clone)]
pub struct CosmosTestWallet {
    pub id: String,
    pub address: String,
    pub keypair: Secp256k1KeyPair,
}

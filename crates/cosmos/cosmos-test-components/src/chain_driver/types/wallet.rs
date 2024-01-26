use ibc_relayer::keyring::Secp256k1KeyPair;

#[derive(Clone)]
pub struct CosmosTestWallet {
    pub id: String,
    pub address: String,
    pub keypair: Secp256k1KeyPair,
}

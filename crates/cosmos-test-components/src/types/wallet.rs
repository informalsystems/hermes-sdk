use ibc_relayer::keyring::Secp256k1KeyPair;

pub struct Wallet {
    pub address: String,
    pub keypair: Secp256k1KeyPair,
}

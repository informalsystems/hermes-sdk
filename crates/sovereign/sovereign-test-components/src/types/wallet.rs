use bech32::ToBase32;
use bech32::Variant::Bech32m;
use ed25519_dalek::{SigningKey, VerifyingKey};
use sha2::Digest;
use sha2::Sha256;

pub struct SovereignWallet {
    pub wallet_id: String,
    pub signing_key: SigningKey,
    pub address: String,
}

impl SovereignWallet {
    pub fn generate(wallet_id: &str, account_prefix: &str) -> Result<Self, bech32::Error> {
        let mut rng = rand::thread_rng();
        let signing_key = SigningKey::generate(&mut rng);
        let address =
            public_key_to_sovereign_address(&signing_key.verifying_key(), account_prefix)?;

        Ok(Self {
            wallet_id: wallet_id.to_owned(),
            signing_key,
            address,
        })
    }
}

pub fn public_key_to_sovereign_address(
    public_key: &VerifyingKey,
    account_prefix: &str,
) -> Result<String, bech32::Error> {
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    let key_hash_bytes: [u8; 32] = hasher.finalize().into();
    let base32_bytes = key_hash_bytes.to_base32();
    let address = bech32::encode(account_prefix, base32_bytes, Bech32m)?;
    Ok(address)
}

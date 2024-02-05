use bech32::ToBase32;
use bech32::Variant::Bech32m;
use ed25519_dalek::{SigningKey, VerifyingKey};
use sha2::Digest;
use sha2::Sha256;

pub struct SovereignWallet {
    pub wallet_id: String,
    pub signing_key: SigningKey,
    pub address: String,
    pub address_hash_bytes: [u8; 32],
}

impl SovereignWallet {
    pub fn generate(wallet_id: &str, account_prefix: &str) -> Result<Self, bech32::Error> {
        let mut rng = rand::thread_rng();
        let signing_key = SigningKey::generate(&mut rng);
        let address_hash_bytes = public_key_to_hash_bytes(&signing_key.verifying_key());

        let address = encode_hash_bytes_to_address(&address_hash_bytes, account_prefix)?;

        Ok(Self {
            wallet_id: wallet_id.to_owned(),
            signing_key,
            address,
            address_hash_bytes,
        })
    }
}

pub fn public_key_to_hash_bytes(public_key: &VerifyingKey) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    hasher.finalize().into()
}

pub fn public_key_to_sovereign_address(
    public_key: &VerifyingKey,
    account_prefix: &str,
) -> Result<String, bech32::Error> {
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    let key_hash_bytes: [u8; 32] = hasher.finalize().into();
    encode_hash_bytes_to_address(&key_hash_bytes, account_prefix)
}

pub fn encode_token_address(
    token_name: &str,
    sender: &[u8],
    salt: u64,
    account_prefix: &str,
) -> Result<String, bech32::Error> {
    let mut hasher = Sha256::new();
    hasher.update(sender);
    hasher.update(token_name.as_bytes());
    hasher.update(salt.to_le_bytes());

    let hash: [u8; 32] = hasher.finalize().into();
    encode_hash_bytes_to_address(&hash, account_prefix)
}
pub fn encode_hash_bytes_to_address(
    hash_bytes: &[u8; 32],
    account_prefix: &str,
) -> Result<String, bech32::Error> {
    let base32_bytes = hash_bytes.to_base32();
    let address = bech32::encode(account_prefix, base32_bytes, Bech32m)?;
    Ok(address)
}

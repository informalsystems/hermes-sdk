use bech32::ToBase32;
use bech32::Variant::Bech32m;
use ed25519_dalek::{SigningKey, VerifyingKey};
use sha2::Digest;
use sha2::Sha256;

pub struct SovereignWallet {
    pub signing_key: SigningKey,
    pub address: String,
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

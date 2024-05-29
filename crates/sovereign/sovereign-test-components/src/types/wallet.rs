use bech32::ToBase32;
use bech32::Variant::Bech32m;
use ed25519_dalek::{SigningKey, VerifyingKey};
use hermes_sovereign_rollup_components::types::address::{SovereignAddress, SovereignAddressBytes};
use sha2::{Digest, Sha256};

pub struct SovereignWallet {
    pub wallet_id: String,
    pub signing_key: SigningKey,
    pub address: SovereignAddress,
    pub credential_id: String,
}

impl SovereignWallet {
    pub fn generate(wallet_id: &str, account_prefix: &str) -> Result<Self, bech32::Error> {
        let mut rng = rand::thread_rng();
        let signing_key = SigningKey::generate(&mut rng);
        let address_hash_bytes = public_key_to_hash_bytes(&signing_key.verifying_key());
        let credential_id = public_key_to_credential_id(&signing_key.verifying_key());

        let address = SovereignAddress::new(address_hash_bytes, account_prefix)?;

        Ok(Self {
            wallet_id: wallet_id.to_owned(),
            signing_key,
            address,
            credential_id,
        })
    }
}

pub fn public_key_to_hash_bytes(public_key: &VerifyingKey) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    hasher.finalize().into()
}

/**
   Encode a public key to a Sovereign SDK address with a given account prefix.

   The encoding is based on the `to_address` function at
   <https://github.com/Sovereign-Labs/sovereign-sdk/blob/c9f56b479c6ea17893e282099fcb8ab804c2feb1/module-system/sov-modules-api/src/default_context.rs#L107>.

   Essentiall, we hash the public key raw bytes, and then convert the raw hash bytes
   using bech32.
*/
pub fn public_key_to_sovereign_address(
    public_key: &VerifyingKey,
    account_prefix: &str,
) -> Result<String, bech32::Error> {
    let key_hash_bytes: [u8; 32] = public_key_to_hash_bytes(public_key);
    encode_hash_bytes_to_address(&key_hash_bytes, account_prefix)
}

pub fn public_key_to_credential_id(public_key: &VerifyingKey) -> String {
    let key_hash_bytes: [u8; 32] = public_key_to_hash_bytes(public_key);
    format!("0x{}", hex::encode(key_hash_bytes))
}

/**
   Encode a token with the sender as a Sovereign address.

   This is based on the `get_token_address` function at
   <https://github.com/Sovereign-Labs/sovereign-sdk/blob/c9f56b479c6ea17893e282099fcb8ab804c2feb1/module-system/module-implementations/sov-bank/src/utils.rs#L6>.

   Essentially, we take the hash of the sender hash bytes, token name, and salt.
   We then perform bech32 encoding on the raw hash value.
*/
pub fn encode_token_address(
    token_name: &str,
    sender: &[u8],
    salt: u64,
    account_prefix: &str,
) -> Result<SovereignAddress, bech32::Error> {
    let address_bytes = encode_token_address_bytes(token_name, sender, salt);

    let address = encode_hash_bytes_to_address(&address_bytes, account_prefix)?;

    Ok(SovereignAddress {
        address,
        address_bytes: SovereignAddressBytes {
            addr: address_bytes,
        },
    })
}

pub fn encode_token_address_bytes(token_name: &str, sender: &[u8], salt: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(sender);
    hasher.update(token_name.as_bytes());
    hasher.update(salt.to_le_bytes());

    hasher.finalize().into()
}

pub fn encode_hash_bytes_to_address(
    hash_bytes: &[u8; 32],
    account_prefix: &str,
) -> Result<String, bech32::Error> {
    let base32_bytes = hash_bytes.to_base32();
    let address = bech32::encode(account_prefix, base32_bytes, Bech32m)?;
    Ok(address)
}

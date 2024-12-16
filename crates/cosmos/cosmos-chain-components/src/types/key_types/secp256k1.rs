use core::str::FromStr;

use bech32::{FromBase32, ToBase32};
use bip39::{Language, Mnemonic, Seed};
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::network::Network;
use digest::Digest;
use generic_array::typenum::U32;
use generic_array::GenericArray;
use hdpath::StandardHDPath;
use ripemd::Ripemd160;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::Sha256;
use subtle_encoding::base64;

pub const KEYSTORE_DEFAULT_FOLDER: &str = ".hermes/keys/";
pub const KEYSTORE_DISK_BACKEND: &str = "keyring-test";
pub const KEYSTORE_FILE_EXTENSION: &str = "json";

/// JSON key seed file
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyFile {
    name: String,
    r#type: String,
    address: String,
    pubkey: String,
    mnemonic: String,
}

#[derive(Debug)]
pub enum EncodedPubKey {
    Bech32(Vec<u8>),
    Proto(ProtoAny),
}

impl FromStr for EncodedPubKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to deserialize into a JSON Value.
        let maybe_json: Result<ProtoAny, _> = serde_json::from_str(s);

        match maybe_json {
            Ok(proto) => {
                if proto.tpe != "/cosmos.crypto.secp256k1.PubKey"
                    && !proto.tpe.ends_with(".ethsecp256k1.PubKey")
                {
                    Err(format!("unsupported public key: {}. only secp256k1 pub keys are currently supported", proto.tpe))
                } else {
                    Ok(Self::Proto(proto))
                }
            }
            Err(e) if e.classify() == serde_json::error::Category::Syntax => {
                // Input is not syntactically-correct JSON.
                // Attempt to decode via Bech32, for backwards compatibility with the old format.
                let (_, data, _) = bech32::decode(s)
                    .map_err(|e| format!("cannot generate bech32 account. Cause: {}", e))?;
                let bech32_bytes = Vec::from_base32(&data)
                    .map_err(|e| format!("cannot generate bech32 account. Cause: {}", e))?;
                Ok(Self::Bech32(bech32_bytes))
            }
            Err(e) => Err(format!(
                "cannot deserialize the encoded public key {}. Cause: {}",
                s, e
            )),
        }
    }
}

/// A variant of [`EncodedPubKey`].
/// A Protobuf `Any`, having support for deserialization from
/// JSON + base64 (see `deserialize_key`).
#[derive(Debug, Deserialize)]
pub struct ProtoAny {
    #[serde(alias = "@type")]
    tpe: String,

    #[serde(deserialize_with = "deserialize_key")]
    _key: Vec<u8>,
}

/// This method is the workhorse for deserializing
/// the `key` field from a public key.
fn deserialize_key<'de, D>(deser: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    // The key is a byte array that is base64-encoded
    // and then marshalled into a JSON String.
    let based64_encoded: Result<String, _> = Deserialize::deserialize(deser);
    let value = base64::decode(based64_encoded?)
        .map_err(|e| serde::de::Error::custom(format!("error in decoding: {e}")))?;

    Ok(value)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Secp256k1KeyPair {
    private_key: SecretKey,
    pub public_key: PublicKey,
    //address: [u8; 20],
    account: String,
}

impl Secp256k1KeyPair {
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let hashed_message: GenericArray<u8, U32> = Sha256::digest(message);

        assert!(hashed_message.len() == 32);

        // SAFETY: hashed_message is 32 bytes, as expected in `Message::from_slice`.
        let message = Message::from_digest_slice(&hashed_message).map_err(|e| {
            format!("failed to create digest Message from hashed messaage. Cause: {e}")
        })?;

        Ok(Secp256k1::signing_only()
            .sign_ecdsa(&message, &self.private_key)
            .serialize_compact()
            .to_vec())
    }

    pub fn account(&self) -> String {
        self.account.to_string()
    }

    pub fn from_key_file(key_file: KeyFile, hd_path: &StandardHDPath) -> Result<Self, String> {
        let mnemonic = Mnemonic::from_phrase(&key_file.mnemonic, Language::English)
            .map_err(|e| format!("invalid mnemonic. Cause: {e}"))?;

        let seed = Seed::new(&mnemonic, "");

        let base_key = Xpriv::new_master(Network::Bitcoin, seed.as_bytes()).map_err(|e| {
            format!("cannot generate secp256k1 private key from BIP-32 seed. Cause: {e}")
        })?;

        let child_numbers = vec![
            ChildNumber::from_hardened_idx(hd_path.purpose().as_value().as_number())
                .expect("Purpose is not Hardened"),
            ChildNumber::from_hardened_idx(hd_path.coin_type()).expect("Coin Type is not Hardened"),
            ChildNumber::from_hardened_idx(hd_path.account()).expect("Account is not Hardened"),
            ChildNumber::from_normal_idx(hd_path.change()).expect("Change is Hardened"),
            ChildNumber::from_normal_idx(hd_path.index()).expect("Index is Hardened"),
        ];

        let path = DerivationPath::from(child_numbers);
        let private_key = base_key
            .derive_priv(&Secp256k1::new(), &path)
            .map_err(|e| {
                format!("cannot generate secp256k1 private key from BIP-32 seed. Cause: {e}")
            })?;

        let derived_pubkey = Xpub::from_priv(&Secp256k1::signing_only(), &private_key);

        Ok(Self {
            private_key: private_key.private_key,
            public_key: derived_pubkey.public_key,
            account: key_file.address,
        })
    }

    pub fn from_seed_file(contents: &str, hd_path: &StandardHDPath) -> Result<Self, String> {
        let key_file =
            serde_json::from_str(contents).map_err(|e| format!("error encoding key. Cause {e}"))?;
        Self::from_key_file(key_file, hd_path)
    }

    pub fn from_mnemonic(
        mnemonic: &str,
        hd_path: &StandardHDPath,
        account_prefix: &str,
    ) -> Result<Self, String> {
        let private_key = Self::private_key_from_mnemonic(mnemonic, hd_path)?;
        let public_key = Xpub::from_priv(&Secp256k1::signing_only(), &private_key);
        let address: [u8; 20] =
            Ripemd160::digest(Sha256::digest(public_key.public_key.serialize())).into();
        let account = bech32::encode(account_prefix, address.to_base32(), bech32::Variant::Bech32)
            .map_err(|e| format!("error encoding address to base 32. Cause {e}"))?;

        Ok(Self {
            private_key: private_key.private_key,
            public_key: public_key.public_key,
            account,
        })
    }

    fn private_key_from_mnemonic(
        mnemonic_words: &str,
        hd_path: &StandardHDPath,
    ) -> Result<Xpriv, String> {
        let mnemonic = Mnemonic::from_phrase(mnemonic_words, Language::English)
            .map_err(|e| format!("invalid mnemonics. Cause: {e}"))?;

        let seed = Seed::new(&mnemonic, "");

        let base_key = Xpriv::new_master(Network::Bitcoin, seed.as_bytes())
            .map_err(|e| format!("failed to create new master key from seed. Cause: {e}"))?;
        let child_numbers = vec![
            ChildNumber::from_hardened_idx(hd_path.purpose().as_value().as_number())
                .expect("Purpose is not Hardened"),
            ChildNumber::from_hardened_idx(hd_path.coin_type()).expect("Coin Type is not Hardened"),
            ChildNumber::from_hardened_idx(hd_path.account()).expect("Account is not Hardened"),
            ChildNumber::from_normal_idx(hd_path.change()).expect("Change is Hardened"),
            ChildNumber::from_normal_idx(hd_path.index()).expect("Index is Hardened"),
        ];

        let path = DerivationPath::from(child_numbers);
        let private_key = base_key
            .derive_priv(&Secp256k1::new(), &path)
            .map_err(|e| {
                format!("failed to derive an extended private key from a path. Cause: {e}")
            })?;

        Ok(private_key)
    }
}

use core::fmt::{Debug, Display};

use hex::{FromHex, ToHex};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

pub struct TxHash(pub [u8; 32]);

impl TxHash {
    pub fn from_signed_tx_bytes(tx_bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(tx_bytes);
        let hash_bytes = hasher.finalize().into();
        Self(hash_bytes)
    }
}

impl Display for TxHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        write!(f, "{}", self.0.encode_hex::<String>())?;
        Ok(())
    }
}

impl Debug for TxHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;
        Ok(())
    }
}

impl Serialize for TxHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TxHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_string_with_prefix = String::deserialize(deserializer)?;

        let hash_string = hex_string_with_prefix.trim_start_matches("0x");
        let hash_bytes = <[u8; 32]>::from_hex(hash_string).map_err(D::Error::custom)?;

        Ok(Self(hash_bytes))
    }
}

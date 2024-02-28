use core::fmt::Debug;

use bech32::ToBase32;
use bech32::Variant::Bech32m;
use borsh::{BorshDeserialize, BorshSerialize};
use hex::ToHex;

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct SovereignAddressBytes {
    pub addr: [u8; 32],
}

impl Debug for SovereignAddressBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SovAddressBytes({})", self.addr.encode_hex::<String>())?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct SovereignAddress {
    pub address: String,
    pub address_bytes: SovereignAddressBytes,
}

impl SovereignAddress {
    pub fn new(address_bytes: [u8; 32], account_prefix: &str) -> Result<Self, bech32::Error> {
        let address = encode_address_bytes_to_address(&address_bytes, account_prefix)?;

        Ok(Self {
            address,
            address_bytes: SovereignAddressBytes {
                addr: address_bytes,
            },
        })
    }
}

pub fn encode_address_bytes_to_address(
    address_bytes: &[u8; 32],
    account_prefix: &str,
) -> Result<String, bech32::Error> {
    let base32_bytes = address_bytes.to_base32();
    let address = bech32::encode(account_prefix, base32_bytes, Bech32m)?;
    Ok(address)
}

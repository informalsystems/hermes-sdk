use bech32::ToBase32;
use bech32::Variant::Bech32m;
use borsh::BorshSerialize;

#[derive(Clone, BorshSerialize)]
pub struct SovereignAddressBytes {
    pub addr: [u8; 32],
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

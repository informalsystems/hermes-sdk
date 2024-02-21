use borsh::BorshSerialize;
use ed25519_dalek::{Signature, VerifyingKey};

#[derive(BorshSerialize)]
pub struct SovereignTransaction {
    signature: SerializeSignature,
    pub_key: SerializePublicKey,
    runtime_msg: Vec<u8>,
    chain_id: u64,
    gas_tip: u64,
    gas_limit: u64,
    max_gas_price: Option<[u64; 2]>,
    nonce: u64,
}

pub struct SerializePublicKey(pub VerifyingKey);

pub struct SerializeSignature(pub Signature);

impl BorshSerialize for SerializePublicKey {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())
    }
}

impl BorshSerialize for SerializeSignature {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.0.to_bytes())
    }
}

use borsh::BorshSerialize;
use ed25519_dalek::{Signature, VerifyingKey};

#[derive(BorshSerialize)]
pub struct SovereignTransaction {
    pub signature: SerializeSignature,
    pub pub_key: SerializePublicKey,
    pub runtime_msg: Vec<u8>,
    pub chain_id: u64,
    pub gas_tip: u64,
    pub gas_limit: u64,
    pub max_gas_price: Option<[u64; 2]>,
    pub nonce: u64,
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

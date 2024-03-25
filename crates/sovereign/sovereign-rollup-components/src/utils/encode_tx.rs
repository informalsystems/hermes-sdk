use borsh::ser::BorshSerialize;
use ed25519_dalek::{Signer, SigningKey};

use crate::types::tx::transaction::{SerializePublicKey, SerializeSignature, SovereignTransaction};

pub fn encode_sovereign_tx_sign_bytes(
    mut message: Vec<u8>,
    chain_id: u64,
    gas_tip: u64,
    gas_limit: u64,
    nonce: u64,
) -> Vec<u8> {
    message.extend(&chain_id.to_le_bytes());
    message.extend(gas_tip.to_le_bytes());
    message.extend(gas_limit.to_le_bytes());
    message.extend(nonce.to_le_bytes());

    // Do not encode max_gas_price for now
    message.push(0);

    message
}

pub fn sign_sovereign_tx(
    signing_key: &SigningKey,
    message: Vec<u8>,
    chain_id: u64,
    gas_tip: u64,
    gas_limit: u64,
    nonce: u64,
) -> SovereignTransaction {
    let sign_bytes =
        encode_sovereign_tx_sign_bytes(message.clone(), chain_id, gas_tip, gas_limit, nonce);

    let signature = signing_key.sign(&sign_bytes);
    let public_key = signing_key.verifying_key();

    SovereignTransaction {
        signature: SerializeSignature(signature),
        pub_key: SerializePublicKey(public_key),
        runtime_msg: message,
        chain_id,
        gas_tip,
        gas_limit,
        max_gas_price: None,
        nonce,
    }
}

pub fn encode_and_sign_sovereign_tx(
    signing_key: &SigningKey,
    message: Vec<u8>,
    chain_id: u64,
    gas_tip: u64,
    gas_limit: u64,
    nonce: u64,
) -> Result<Vec<u8>, std::io::Error> {
    let transaction = sign_sovereign_tx(signing_key, message, chain_id, gas_tip, gas_limit, nonce);

    transaction.try_to_vec()
}

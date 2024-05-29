use borsh::ser::BorshSerialize;
use ed25519_dalek::{Signer, SigningKey};

use crate::types::tx::transaction::{
    SerializePublicKey, SerializeSignature, SovereignTransaction, UnsignedSovereignTransaction,
};

pub fn sign_sovereign_tx(
    signing_key: &SigningKey,
    message: Vec<u8>,
    chain_id: u64,
    max_priority_fee_bips: u64,
    max_fee: u64,
    nonce: u64,
) -> Result<SovereignTransaction, std::io::Error> {
    let unsigned_tx = UnsignedSovereignTransaction {
        runtime_msg: message.clone(),
        chain_id,
        max_priority_fee_bips,
        max_fee,
        nonce,
        gas_limit: None,
    };

    let sign_bytes = BorshSerialize::try_to_vec(&unsigned_tx)?;

    let signature = signing_key.sign(&sign_bytes);
    let public_key = signing_key.verifying_key();

    Ok(SovereignTransaction {
        signature: SerializeSignature(signature),
        pub_key: SerializePublicKey(public_key),
        runtime_msg: message,
        chain_id,
        max_priority_fee_bips,
        max_fee,
        gas_limit: None,
        nonce,
    })
}

pub fn encode_and_sign_sovereign_tx(
    signing_key: &SigningKey,
    message: Vec<u8>,
    chain_id: u64,
    max_priority_fee_bips: u64,
    max_fee: u64,
    nonce: u64,
) -> Result<Vec<u8>, std::io::Error> {
    let transaction = sign_sovereign_tx(
        signing_key,
        message,
        chain_id,
        max_priority_fee_bips,
        max_fee,
        nonce,
    )?;

    transaction.try_to_vec()
}

pub fn encode_sovereign_tx(
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

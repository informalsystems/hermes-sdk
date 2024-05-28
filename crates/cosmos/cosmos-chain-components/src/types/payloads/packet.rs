use ibc_relayer_types::Height;

pub struct CosmosAckPacketPayload {
    pub ack: Vec<u8>,
    pub update_height: Height,
    pub proof_acked: Vec<u8>,
}

pub struct CosmosTimeoutUnorderedPacketPayload {
    pub update_height: Height,
    pub proof_unreceived: Vec<u8>,
}

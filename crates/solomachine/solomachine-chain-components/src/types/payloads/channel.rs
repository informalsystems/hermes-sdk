use ibc::core::channel::types::channel::Order;
use ibc::core::channel::types::Version;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ConnectionId;
use secp256k1::ecdsa::Signature;

pub struct SolomachineChannelOpenTryPayload {
    pub ordering: Order,
    pub connection_hops: Vec<ConnectionId>,
    pub version: Version,
    pub update_height: Height,
    pub proof_init: Signature,
}

pub struct SolomachineChannelOpenAckPayload {
    pub version: Version,
    pub update_height: Height,
    pub proof_try: Signature,
}

pub struct SolomachineChannelOpenConfirmPayload {
    pub update_height: Height,
    pub proof_ack: Signature,
}

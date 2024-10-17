use crate::types::channel_id::MockChannelId;
use crate::types::nonce::MockNonce;
use crate::types::tagged::Tagged;

pub enum MockCommitmentPath<Chain, Counterparty> {
    ReceivePacket {
        src_channel_id: Tagged<Counterparty, Chain, MockChannelId>,
        dst_channel_id: Tagged<Chain, Counterparty, MockChannelId>,
        nonce: Tagged<Counterparty, Chain, MockNonce>,
    },
    SendPacket {
        src_channel_id: Tagged<Chain, Counterparty, MockChannelId>,
        dst_channel_id: Tagged<Counterparty, Chain, MockChannelId>,
        nonce: Tagged<Chain, Counterparty, MockNonce>,
    },
}

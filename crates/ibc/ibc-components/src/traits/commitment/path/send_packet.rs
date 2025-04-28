use hermes_chain_type_components::traits::HasChannelIdType;
use hermes_prelude::*;

use crate::traits::types::commitment::path::HasCommitmentPathType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::types::tags::commitment::send::SendPacket;

#[cgp_component {
  provider: SendPacketCommitmentPathBuilder,
  context: Chain,
}]
pub trait CanBuildSendPacketCommitmentPath<Counterparty>:
    HasChannelIdType<Counterparty>
    + HasPacketNonceType<Counterparty>
    + HasCommitmentPathType<SendPacket>
    + HasAsyncErrorType
where
    Counterparty: HasChannelIdType<Self>,
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_send_packet_commitment_path(
        src_channel_id: &Self::ChannelId,
        dst_channel_id: &Counterparty::ChannelId,
        nonce: &Self::PacketNonce,
    ) -> Result<Self::CommitmentPath, Self::Error>;
}

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::commitment::path::HasCommitmentPathType;
use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive_component(ReceivePacketCommitmentPathBuilderComponent, ReceivePacketCommitmentPathBuilder<Chain>)]
pub trait CanBuildReceivePacketCommitmentPath<Counterparty>:
    HasCommitmentPathType + HasChannelIdType<Counterparty> + HasErrorType
where
    Counterparty: HasChannelIdType<Self> + HasPacketNonceType<Self>,
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_receive_packet_commitment_path(
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        nonce: &Counterparty::PacketNonce,
    ) -> Result<Self::CommitmentPath, Self::Error>;
}
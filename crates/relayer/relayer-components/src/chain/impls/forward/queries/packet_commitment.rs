use cgp::core::error::CanRaiseError;
use cgp::core::inner::HasInner;

use crate::chain::traits::queries::packet_commitment::{
    CanQueryPacketCommitment, PacketCommitmentQuerier,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packets::receive::HasPacketCommitmentType;
use crate::chain::traits::types::proof::HasCommitmentProofType;

pub struct ForwardQueryPacketCommitment;

impl<Chain, InChain, Counterparty> PacketCommitmentQuerier<Chain, Counterparty>
    for ForwardQueryPacketCommitment
where
    Chain: HasInner<Inner = InChain>
        + HasIbcChainTypes<Counterparty>
        + HasPacketCommitmentType<Counterparty>
        + HasCommitmentProofType
        + CanRaiseError<InChain::Error>,
    InChain: CanQueryPacketCommitment<
        Counterparty,
        PacketCommitment = Chain::PacketCommitment,
        CommitmentProof = Chain::CommitmentProof,
        ChannelId = Chain::ChannelId,
        Sequence = Chain::Sequence,
        PortId = Chain::PortId,
        Height = Chain::Height,
    >,
{
    async fn query_packet_commitment(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Chain::Sequence,
        height: &Chain::Height,
    ) -> Result<(Chain::PacketCommitment, Chain::CommitmentProof), Chain::Error> {
        chain
            .inner()
            .query_packet_commitment(channel_id, port_id, sequence, height)
            .await
            .map_err(Chain::raise_error)
    }
}

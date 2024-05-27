use cgp_core::{Async, CanRaiseError, HasInner};

use crate::chain::traits::queries::packet_acknowledgement::{
    CanQueryPacketAcknowledgement, PacketAcknowledgementQuerier,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packets::ack::HasAcknowledgementType;
use crate::chain::traits::types::proof::HasCommitmentProofType;

pub struct ForwardQueryPacketAcknowledgement;

impl<Chain, InChain, Counterparty, Sequence> PacketAcknowledgementQuerier<Chain, Counterparty>
    for ForwardQueryPacketAcknowledgement
where
    Chain: HasInner<Inner = InChain>
        + HasIbcChainTypes<Counterparty>
        + HasAcknowledgementType<Counterparty>
        + HasCommitmentProofType
        + CanRaiseError<InChain::Error>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence>
        + HasIbcChainTypes<InChain, Sequence = Sequence>,
    InChain: CanQueryPacketAcknowledgement<
        Counterparty,
        Acknowledgement = Chain::Acknowledgement,
        CommitmentProof = Chain::CommitmentProof,
        ChannelId = Chain::ChannelId,
        PortId = Chain::PortId,
        Height = Chain::Height,
    >,
    Sequence: Async,
{
    async fn query_packet_acknowledgement(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Sequence,
        height: &Chain::Height,
    ) -> Result<(Chain::Acknowledgement, Chain::CommitmentProof), Chain::Error> {
        chain
            .inner()
            .query_packet_acknowledgement(channel_id, port_id, sequence, height)
            .await
            .map_err(Chain::raise_error)
    }
}

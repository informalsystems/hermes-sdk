use cgp::core::error::CanRaiseError;
use cgp::core::inner::HasInner;
use cgp::core::Async;

use crate::traits::queries::packet_receipt::{CanQueryPacketReceipt, PacketReceiptQuerier};
use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::packets::timeout::HasPacketReceiptType;
use crate::traits::types::proof::HasCommitmentProofType;

pub struct ForwardQueryPacketReceipt;

impl<Chain, InChain, Counterparty, Sequence> PacketReceiptQuerier<Chain, Counterparty>
    for ForwardQueryPacketReceipt
where
    Chain: HasInner<Inner = InChain>
        + HasIbcChainTypes<Counterparty>
        + HasPacketReceiptType<Counterparty>
        + HasCommitmentProofType
        + CanRaiseError<InChain::Error>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence>
        + HasIbcChainTypes<InChain, Sequence = Sequence>,
    InChain: CanQueryPacketReceipt<
        Counterparty,
        PacketReceipt = Chain::PacketReceipt,
        CommitmentProof = Chain::CommitmentProof,
        ChannelId = Chain::ChannelId,
        PortId = Chain::PortId,
        Height = Chain::Height,
    >,
    Sequence: Async,
{
    async fn query_packet_receipt(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequence: &Sequence,
        height: &Chain::Height,
    ) -> Result<(Chain::PacketReceipt, Chain::CommitmentProof), Chain::Error> {
        chain
            .inner()
            .query_packet_receipt(channel_id, port_id, sequence, height)
            .await
            .map_err(Chain::raise_error)
    }
}

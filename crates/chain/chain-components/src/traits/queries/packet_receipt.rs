use cgp::prelude::*;

use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::packets::timeout::HasPacketReceiptType;
use crate::traits::types::proof::HasCommitmentProofType;

#[derive_component(PacketReceiptQuerierComponent, PacketReceiptQuerier<Chain>)]
#[async_trait]
pub trait CanQueryPacketReceipt<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasPacketReceiptType<Counterparty>
    + HasCommitmentProofType
    + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    async fn query_packet_receipt(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequence: &Counterparty::Sequence,
        height: &Self::Height,
    ) -> Result<(Self::PacketReceipt, Self::CommitmentProof), Self::Error>;
}

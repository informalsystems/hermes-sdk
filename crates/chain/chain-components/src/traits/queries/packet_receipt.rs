use hermes_chain_type_components::traits::{
    CanUseCounterparty, HasChannelIdType, HasHeightType, HasPortIdType, HasSequenceType,
};
use hermes_prelude::*;

use crate::traits::{HasCommitmentProofType, HasPacketReceiptType};
use crate::types::aliases::SequenceOf;

#[cgp_component {
  provider: PacketReceiptQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketReceipt<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasPacketReceiptType<Counterparty>
    + HasCommitmentProofType
    + HasAsyncErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasSequenceType<Self>>
{
    async fn query_packet_receipt(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequence: &SequenceOf<Counterparty, Self>,
        height: &Self::Height,
    ) -> Result<(Option<Self::PacketReceipt>, Self::CommitmentProof), Self::Error>;
}

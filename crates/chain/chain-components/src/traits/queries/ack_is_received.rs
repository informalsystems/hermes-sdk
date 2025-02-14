use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

#[cgp_component {
    provider: AckIsReceivedQuerier,
    context: Chain,
}]
#[async_trait]
pub trait CanQueryAckIsReceived<Counterparty>:
    HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
    + HasAsyncErrorType
{
    async fn query_ack_is_received(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        sequence: &Self::Sequence,
    ) -> Result<bool, Self::Error>;
}

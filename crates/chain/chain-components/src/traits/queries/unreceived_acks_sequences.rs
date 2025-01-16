use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

#[cgp_component {
  provider: UnreceivedAcksSequencesQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryUnreceivedAcksSequences<Counterparty>:
    HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
    + HasAsyncErrorType
{
    /// Performs a query about which IBC packets in the specified list has not
    /// been acknowledged. Returns the sequence numbers of the packets that were not
    /// acknowledged.
    ///
    /// For example, given a request with the sequence numbers `[5,6,7,8]`, a
    /// response of `[7,8]` would indicate that packets 5 & 6 were acknowledged,
    /// while packets 7, 8 were not.
    async fn query_unreceived_acknowledgments_sequences(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        packet_ack_sequences: &[Self::Sequence],
    ) -> Result<Vec<Self::Sequence>, Self::Error>;
}

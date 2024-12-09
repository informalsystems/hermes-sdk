use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::types::ibc::HasIbcChainTypes;

#[cgp_component {
  provider: UnreceivedAcksSequencesQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryUnreceivedAcksSequences<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
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

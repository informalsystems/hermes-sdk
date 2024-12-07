use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::types::ibc::HasIbcChainTypes;

#[cgp_component {
  name: UnreceivedPacketSequencesQuerierComponent,
  provider: UnreceivedPacketSequencesQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryUnreceivedPacketSequences<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    /// Given a list of counterparty commitment sequences,
    /// return a filtered list of sequences which the chain
    /// has not received the packet from the counterparty chain.
    async fn query_unreceived_packet_sequences(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequences: &[Counterparty::Sequence],
    ) -> Result<Vec<Counterparty::Sequence>, Self::Error>;
}

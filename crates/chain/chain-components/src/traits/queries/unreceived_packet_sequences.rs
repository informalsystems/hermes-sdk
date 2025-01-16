use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::counterparty::CanUseCounterparty;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

use crate::types::aliases::SequenceOf;

#[cgp_component {
  provider: UnreceivedPacketSequencesQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryUnreceivedPacketSequences<Counterparty>:
    HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasAsyncErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasSequenceType<Self>>
{
    /// Given a list of counterparty commitment sequences,
    /// return a filtered list of sequences which the chain
    /// has not received the packet from the counterparty chain.
    async fn query_unreceived_packet_sequences(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequences: &[SequenceOf<Counterparty, Self>],
    ) -> Result<Vec<SequenceOf<Counterparty, Self>>, Self::Error>;
}

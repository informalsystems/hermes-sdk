use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::ibc::sequence::HasSequenceType;

#[cgp_component {
  provider: PacketAcknowledgementsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketAcknowledgements<Counterparty>:
    HasHeightType + HasChannelIdType<Counterparty> + HasPortIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasSequenceType<Self>,
{
    /// Query the sequences of the packets that the chain has committed to be
    /// sent to the counterparty chain, of which the full packet relaying is not
    /// yet completed. Once the chain receives the ack from the counterparty
    /// chain, a given sequence should be removed from the packet commitment list.
    async fn query_packet_acknowlegements(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        sequences: &[Counterparty::Sequence],
    ) -> Result<Option<(Vec<Counterparty::Sequence>, Self::Height)>, Self::Error>;
}

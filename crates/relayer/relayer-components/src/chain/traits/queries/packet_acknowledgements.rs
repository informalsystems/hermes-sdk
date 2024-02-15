use alloc::vec::Vec;

use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(PacketAcknowledgementsQuerierComponent, PacketAcknowledgementsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryPacketAcknowledgements<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
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

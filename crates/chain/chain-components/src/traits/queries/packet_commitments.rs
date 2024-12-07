use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::types::ibc::HasIbcChainTypes;

#[cgp_component {
  name: PacketCommitmentsQuerierComponent,
  provider: PacketCommitmentsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketCommitments<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
{
    /// Query the sequences of the packets that the chain has committed to be
    /// sent to the counterparty chain, of which the full packet relaying is not
    /// yet completed. Once the chain receives the ack from the counterparty
    /// chain, a given sequence should be removed from the packet commitment list.
    async fn query_packet_commitments(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<(Vec<Self::Sequence>, Self::Height), Self::Error>;
}

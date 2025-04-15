use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasChannelIdType, HasPortIdType, HasSequenceType};

#[cgp_component {
  provider: PacketIsReceivedQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryPacketIsReceived<Counterparty>:
    HasChannelIdType<Counterparty> + HasPortIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasSequenceType<Self>,
{
    async fn query_packet_is_received(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        sequence: &Counterparty::Sequence,
    ) -> Result<bool, Self::Error>;
}

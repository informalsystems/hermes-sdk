use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_chain_type_components::traits::HasOutgoingPacketType;

use crate::traits::{HasMessageType, HasReceivePacketPayloadType};

#[cgp_component {
  provider: ReceivePacketMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildReceivePacketMessage<Counterparty>: HasMessageType + HasAsyncErrorType
where
    Counterparty: HasOutgoingPacketType<Self> + HasReceivePacketPayloadType<Self>,
{
    async fn build_receive_packet_message(
        &self,
        packet: &Counterparty::OutgoingPacket,
        payload: Counterparty::ReceivePacketPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_provider(ReceivePacketMessageBuilderComponent)]
impl<Chain, Counterparty, Components> ReceivePacketMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasMessageType + HasAsyncErrorType,
    Counterparty: HasOutgoingPacketType<Chain> + HasReceivePacketPayloadType<Chain>,
    Components: DelegateComponent<Counterparty>,
    Components::Delegate: ReceivePacketMessageBuilder<Chain, Counterparty>,
{
    async fn build_receive_packet_message(
        chain: &Chain,
        packet: &Counterparty::OutgoingPacket,
        payload: Counterparty::ReceivePacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Components::Delegate::build_receive_packet_message(chain, packet, payload).await
    }
}

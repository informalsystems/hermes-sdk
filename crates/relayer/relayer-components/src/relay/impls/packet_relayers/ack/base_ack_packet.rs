use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::packets::ack::AcknowledgementOf;
use hermes_chain_components::types::aliases::HeightOf;

use crate::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use crate::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::components::default::relay::re_exports::AckPacketRelayerComponent;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayClientIds, PacketOf};
use crate::relay::traits::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::packet_relayers::ack_packet::AckPacketRelayer;
use crate::relay::traits::target::{HasSourceTargetChainTypes, SourceTarget};

/// The minimal component that can send an acknowledgement packet.
/// Ack packet relayers with more capabilities can be implemented
/// on top of this base type.
pub struct BaseAckPacketRelayer;

#[cgp_provider(AckPacketRelayerComponent)]
impl<Relay> AckPacketRelayer<Relay> for BaseAckPacketRelayer
where
    Relay: HasSourceTargetChainTypes
        + HasRelayClientIds
        + CanRaiseRelayChainErrors
        + CanSendSingleIbcMessage<MainSink, SourceTarget>,
    Relay::SrcChain: CanQueryClientStateWithLatestHeight<Relay::DstChain>
        + CanBuildAckPacketMessage<Relay::DstChain>
        + HasOutgoingPacketType<Relay::DstChain>,
    Relay::DstChain: HasClientStateType<Relay::SrcChain>
        + CanBuildAckPacketPayload<Relay::SrcChain>
        + HasWriteAckEvent<Relay::SrcChain>,
{
    async fn relay_ack_packet(
        relay: &Relay,
        destination_height: &HeightOf<Relay::DstChain>,
        packet: &PacketOf<Relay>,
        ack: &AcknowledgementOf<Relay::DstChain, Relay::SrcChain>,
    ) -> Result<(), Relay::Error> {
        let src_client_state = relay
            .src_chain()
            .query_client_state_with_latest_height(PhantomData, relay.src_client_id())
            .await
            .map_err(Relay::raise_error)?;

        let payload = relay
            .dst_chain()
            .build_ack_packet_payload(&src_client_state, destination_height, packet, ack)
            .await
            .map_err(Relay::raise_error)?;

        let message = relay
            .src_chain()
            .build_ack_packet_message(packet, payload)
            .await
            .map_err(Relay::raise_error)?;

        relay.send_message(SourceTarget, message).await?;

        Ok(())
    }
}

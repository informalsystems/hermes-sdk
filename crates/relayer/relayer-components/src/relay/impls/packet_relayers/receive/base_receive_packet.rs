use core::marker::PhantomData;

use hermes_chain_components::traits::extract_data::CanExtractFromEvent;
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;

use crate::chain::traits::message_builders::receive_packet::CanBuildReceivePacketMessage;
use crate::chain::traits::payload_builders::receive_packet::CanBuildReceivePacketPayload;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasDstClientId, HasRelayChains, PacketOf,
};
use crate::relay::traits::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayer;
use crate::relay::traits::target::{DestinationTarget, HasDestinationTargetChainTypes};

pub struct BaseReceivePacketRelayer;

impl<Relay, AckEvent> ReceivePacketRelayer<Relay> for BaseReceivePacketRelayer
where
    Relay: HasRelayChains
        + HasDestinationTargetChainTypes
        + HasDstClientId
        + CanSendSingleIbcMessage<MainSink, DestinationTarget>
        + CanRaiseRelayChainErrors,
    Relay::SrcChain: CanBuildReceivePacketPayload<Relay::DstChain>,
    Relay::DstChain: CanQueryClientStateWithLatestHeight<Relay::SrcChain>
        + CanBuildReceivePacketMessage<Relay::SrcChain>
        + HasMessageResponseEvents
        + HasWriteAckEvent<Relay::SrcChain, WriteAckEvent = AckEvent>
        + CanExtractFromEvent<AckEvent>,
{
    async fn relay_receive_packet(
        relay: &Relay,
        source_height: &HeightOf<Relay::SrcChain>,
        packet: &PacketOf<Relay>,
    ) -> Result<Option<AckEvent>, Relay::Error> {
        let dst_chain = relay.dst_chain();

        let src_client_state = dst_chain
            .query_client_state_with_latest_height(PhantomData, relay.dst_client_id())
            .await
            .map_err(Relay::raise_error)?;

        let payload = relay
            .src_chain()
            .build_receive_packet_payload(&src_client_state, source_height, packet)
            .await
            .map_err(Relay::raise_error)?;

        let message = dst_chain
            .build_receive_packet_message(packet, payload)
            .await
            .map_err(Relay::raise_error)?;

        let response = relay.send_message(DestinationTarget, message).await?;

        let ack_event = Relay::DstChain::message_response_events(&response)
            .iter()
            .find_map(|event| dst_chain.try_extract_from_event(PhantomData, event));

        Ok(ack_event)
    }
}

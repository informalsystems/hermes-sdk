use crate::chain::traits::message_builders::receive_packet::CanBuildReceivePacketMessage;
use crate::chain::traits::payload_builders::receive_packet::CanBuildReceivePacketPayload;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, PacketOf};
use crate::relay::traits::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::packet_relayers::receive_packet::ReceivePacketRelayer;
use crate::relay::traits::target::DestinationTarget;

pub struct BaseReceivePacketRelayer;

impl<Relay, AckEvent> ReceivePacketRelayer<Relay> for BaseReceivePacketRelayer
where
    Relay: HasRelayChains
        + CanSendSingleIbcMessage<MainSink, DestinationTarget>
        + CanRaiseRelayChainErrors,
    Relay::SrcChain: CanBuildReceivePacketPayload<Relay::DstChain>,
    Relay::DstChain: CanQueryClientStateWithLatestHeight<Relay::SrcChain>
        + CanBuildReceivePacketMessage<Relay::SrcChain>
        + HasWriteAckEvent<Relay::SrcChain, WriteAckEvent = AckEvent>,
{
    async fn relay_receive_packet(
        relay: &Relay,
        source_height: &HeightOf<Relay::SrcChain>,
        packet: &PacketOf<Relay>,
    ) -> Result<Option<AckEvent>, Relay::Error> {
        let src_client_state = relay
            .dst_chain()
            .query_client_state_with_latest_height(relay.dst_client_id())
            .await
            .map_err(Relay::raise_error)?;

        let payload = relay
            .src_chain()
            .build_receive_packet_payload(&src_client_state, source_height, packet)
            .await
            .map_err(Relay::raise_error)?;

        let message = relay
            .dst_chain()
            .build_receive_packet_message(packet, payload)
            .await
            .map_err(Relay::raise_error)?;

        let events = relay.send_message(DestinationTarget, message).await?;

        let ack_event = events
            .iter()
            .find_map(Relay::DstChain::try_extract_write_ack_event);

        Ok(ack_event)
    }
}

use cgp_core::Async;

use crate::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use crate::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::packet_relayers::ack_packet::AckPacketRelayer;
use crate::relay::traits::target::SourceTarget;

/// The minimal component that can send an acknowledgement packet.
/// Ack packet relayers with more capabilities can be implemented
/// on top of this base type.
pub struct BaseAckPacketRelayer;

impl<Relay, SrcChain, DstChain, Packet> AckPacketRelayer<Relay> for BaseAckPacketRelayer
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain, Packet = Packet>
        + CanRaiseRelayChainErrors,
    Relay: CanSendSingleIbcMessage<MainSink, SourceTarget>,
    SrcChain: CanQueryClientStateWithLatestHeight<DstChain>
        + CanBuildAckPacketMessage<DstChain>
        + HasIbcPacketTypes<DstChain, OutgoingPacket = Packet>,
    DstChain: HasClientStateType<SrcChain>
        + CanBuildAckPacketPayload<SrcChain>
        + HasIbcPacketTypes<SrcChain, IncomingPacket = Packet>
        + HasWriteAckEvent<SrcChain>,
    Packet: Async,
{
    async fn relay_ack_packet(
        relay: &Relay,
        destination_height: &DstChain::Height,
        packet: &Packet,
        ack: &DstChain::Acknowledgement,
    ) -> Result<(), Relay::Error> {
        let src_client_state = relay
            .src_chain()
            .query_client_state_with_latest_height(relay.src_client_id())
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

use cgp_core::{async_trait, Async};

use crate::chain::traits::components::ack_packet_message_builder::CanBuildAckPacketMessage;
use crate::chain::traits::components::ack_packet_payload_builder::CanBuildAckPacketPayload;
use crate::chain::traits::components::client_state_querier::CanQueryClientState;
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::components::ibc_message_sender::{CanSendSingleIbcMessage, MainSink};
use crate::relay::traits::components::packet_relayers::ack_packet::AckPacketRelayer;
use crate::relay::traits::target::SourceTarget;

/// The minimal component that can send an acknowledgement packet.
/// Ack packet relayers with more capabilities can be implemented
/// on top of this base type.
pub struct BaseAckPacketRelayer;

#[async_trait]
impl<Relay, SrcChain, DstChain, Packet> AckPacketRelayer<Relay> for BaseAckPacketRelayer
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain, Packet = Packet>
        + CanRaiseRelayChainErrors,
    Relay: CanSendSingleIbcMessage<MainSink, SourceTarget>,
    SrcChain: CanQueryClientState<DstChain>
        + CanBuildAckPacketMessage<DstChain>
        + HasIbcPacketTypes<DstChain, OutgoingPacket = Packet>,
    DstChain: HasClientStateType<SrcChain>
        + CanBuildAckPacketPayload<SrcChain>
        + HasIbcPacketTypes<SrcChain, IncomingPacket = Packet>,
    Packet: Async,
{
    async fn relay_ack_packet(
        relay: &Relay,
        destination_height: &DstChain::Height,
        packet: &Packet,
        ack: &DstChain::WriteAckEvent,
    ) -> Result<(), Relay::Error> {
        let src_client_state = relay
            .src_chain()
            .query_client_state(relay.src_client_id())
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

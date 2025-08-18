use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_chain_components::traits::{AcknowledgementOf, HasOutgoingPacketType};
use hermes_chain_components::types::aliases::HeightOf;
use hermes_prelude::*;

use crate::chain::traits::{
    CanBuildAckPacketMessage, CanBuildAckPacketPayload, CanQueryClientStateWithLatestHeight,
    HasClientStateType, HasWriteAckEvent,
};
use crate::relay::traits::{
    BatchAckPacketsRelayer, BatchAckPacketsRelayerComponent, CanRaiseRelayChainErrors,
    CanSendIbcMessages, HasRelayClientIds, HasSourceTargetChainTypes, MainSink, PacketOf,
    SourceTarget,
};

/// The minimal component that can send an acknowledgement packet.
/// Ack packet relayers with more capabilities can be implemented
/// on top of this base type.
pub struct BatchedAckPacketsRelayer;

#[cgp_provider(BatchAckPacketsRelayerComponent)]
impl<Relay> BatchAckPacketsRelayer<Relay> for BatchedAckPacketsRelayer
where
    Relay: HasSourceTargetChainTypes
        + HasRelayClientIds
        + CanRaiseRelayChainErrors
        + CanSendIbcMessages<MainSink, SourceTarget>,
    Relay::SrcChain: CanQueryClientStateWithLatestHeight<Relay::DstChain>
        + CanBuildAckPacketMessage<Relay::DstChain>
        + HasOutgoingPacketType<Relay::DstChain>,
    Relay::DstChain: HasClientStateType<Relay::SrcChain>
        + CanBuildAckPacketPayload<Relay::SrcChain>
        + HasWriteAckEvent<Relay::SrcChain>,
{
    async fn relay_ack_packets(
        relay: &Relay,
        destination_heights: Vec<&HeightOf<Relay::DstChain>>,
        packets: Vec<&PacketOf<Relay>>,
        acks: Vec<&AcknowledgementOf<Relay::DstChain, Relay::SrcChain>>,
    ) -> Result<(), Relay::Error> {
        let mut messages = vec![];

        for ((destination_height, packet), ack) in destination_heights
            .iter()
            .zip(packets.iter())
            .zip(acks.iter())
        {
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

            messages.push(message);
        }
        relay.send_messages(SourceTarget, messages).await?;

        Ok(())
    }
}

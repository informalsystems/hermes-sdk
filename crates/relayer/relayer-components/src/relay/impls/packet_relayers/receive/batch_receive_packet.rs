use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_chain_components::traits::{
    CanBuildPacketFromWriteAck, CanBuildReceivePacketMessage, CanBuildReceivePacketPayload,
    CanExtractFromEvent, CanQueryClientStateWithLatestHeight, HasAcknowledgementType,
    HasWriteAckEvent,
};
use hermes_chain_components::types::aliases::HeightOf;
use hermes_chain_type_components::traits::HasMessageResponseEvents;
use hermes_prelude::*;

use crate::relay::traits::{
    BatchReceivePacketsRelayer, BatchReceivePacketsRelayerComponent, CanRaiseRelayChainErrors,
    CanSendIbcMessages, DestinationTarget, HasDestinationTargetChainTypes, HasDstClientId,
    HasRelayChains, MainSink, PacketOf,
};

pub struct BatchedReceivePacketsRelayer;

#[cgp_provider(BatchReceivePacketsRelayerComponent)]
impl<Relay, SrcChain, DstChain> BatchReceivePacketsRelayer<Relay> for BatchedReceivePacketsRelayer
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasDestinationTargetChainTypes
        + HasDstClientId
        + CanSendIbcMessages<MainSink, DestinationTarget>
        + CanRaiseRelayChainErrors,
    SrcChain: CanBuildReceivePacketPayload<DstChain>,
    DstChain: CanQueryClientStateWithLatestHeight<SrcChain>
        + CanBuildReceivePacketMessage<SrcChain>
        + HasMessageResponseEvents
        + HasWriteAckEvent<SrcChain>
        + CanExtractFromEvent<DstChain::WriteAckEvent>
        + HasAcknowledgementType<SrcChain>
        + CanBuildPacketFromWriteAck<SrcChain>,
{
    async fn relay_receive_packets(
        relay: &Relay,
        source_height: &HeightOf<Relay::SrcChain>,
        packets: Vec<&PacketOf<Relay>>,
    ) -> Result<Vec<Option<DstChain::Acknowledgement>>, Relay::Error> {
        if packets.is_empty() {
            return Ok(vec![]);
        }

        let dst_chain = relay.dst_chain();

        let src_client_state = dst_chain
            .query_client_state_with_latest_height(PhantomData, relay.dst_client_id())
            .await
            .map_err(Relay::raise_error)?;

        let mut messages = vec![];

        for packet in packets.iter() {
            let payload = relay
                .src_chain()
                .build_receive_packet_payload(&src_client_state, source_height, packet)
                .await
                .map_err(Relay::raise_error)?;

            let message = dst_chain
                .build_receive_packet_message(packet, payload)
                .await
                .map_err(Relay::raise_error)?;
            messages.push(message);
        }

        let responses = relay.send_messages(DestinationTarget, messages).await?;

        let mut acks = vec![];

        for response in responses.iter() {
            let m_ack_event = Relay::DstChain::message_response_events(response)
                .iter()
                .find_map(|event| dst_chain.try_extract_from_event(PhantomData, event));

            match m_ack_event {
                Some(ack_event) => {
                    let ack = dst_chain
                        .build_ack_from_write_ack_event(&ack_event)
                        .await
                        .map_err(Relay::raise_error)?;

                    acks.push(Some(ack));
                }
                None => {
                    acks.push(None);
                }
            }
        }
        Ok(acks)
    }
}

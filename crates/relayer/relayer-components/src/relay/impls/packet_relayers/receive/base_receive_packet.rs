use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::{
    CanBuildPacketFromWriteAck, CanExtractFromEvent, HasAcknowledgementType,
};
use hermes_chain_type_components::traits::HasMessageResponseEvents;

use crate::chain::traits::{
    CanBuildReceivePacketMessage, CanBuildReceivePacketPayload,
    CanQueryClientStateWithLatestHeight, HasWriteAckEvent,
};
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{
    CanRaiseRelayChainErrors, CanSendSingleIbcMessage, DestinationTarget,
    HasDestinationTargetChainTypes, HasDstClientId, HasRelayChains, MainSink, PacketOf,
    ReceivePacketRelayer, ReceivePacketRelayerComponent,
};

pub struct BaseReceivePacketRelayer;

#[cgp_provider(ReceivePacketRelayerComponent)]
impl<Relay, SrcChain, DstChain> ReceivePacketRelayer<Relay> for BaseReceivePacketRelayer
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasDestinationTargetChainTypes
        + HasDstClientId
        + CanSendSingleIbcMessage<MainSink, DestinationTarget>
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
    async fn relay_receive_packet(
        relay: &Relay,
        source_height: &HeightOf<Relay::SrcChain>,
        packet: &PacketOf<Relay>,
    ) -> Result<Option<DstChain::Acknowledgement>, Relay::Error> {
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

        let m_ack_event = Relay::DstChain::message_response_events(&response)
            .iter()
            .find_map(|event| dst_chain.try_extract_from_event(PhantomData, event));

        match m_ack_event {
            Some(ack_event) => {
                let ack = dst_chain
                    .build_ack_from_write_ack_event(&ack_event)
                    .await
                    .map_err(Relay::raise_error)?;

                Ok(Some(ack))
            }
            None => Ok(None),
        }
    }
}

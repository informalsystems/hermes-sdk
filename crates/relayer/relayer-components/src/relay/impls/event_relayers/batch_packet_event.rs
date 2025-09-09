use alloc::{format, vec};
use core::marker::PhantomData;

use hermes_chain_components::traits::{
    CanBuildMisbehaviourMessage, CanBuildPacketFromSendPacket, CanCheckMisbehaviour,
    CanExtractFromEvent, CanQueryChainHeight, CanQueryClientStateWithLatestHeight,
    CanSendSingleMessage, HasClientIdType, HasClientStateType, HasEvidenceType,
    HasUpdateClientEventFields,
};
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::{LevelDebug, LevelWarn};
use hermes_prelude::*;

use crate::chain::traits::{CanBuildPacketFromWriteAck, HasSendPacketEvent};
use crate::chain::types::aliases::EventOf;
use crate::relay::impls::{MatchPacketDestinationChain, MatchPacketSourceChain};
use crate::relay::traits::{
    BatchEventRelayer, BatchEventRelayerComponent, CanFilterRelayPackets, CanRaiseRelayChainErrors,
    CanRelayBatchAckPackets, CanRelayBatchPackets, DestinationTarget, HasRelayChains,
    HasRelayClientIds, RelayPacketFilter, SourceTarget,
};

pub struct BatchPacketEventRelayer;

#[cgp_provider(BatchEventRelayerComponent)]
impl<Relay, SrcChain, DstChain> BatchEventRelayer<Relay, SourceTarget> for BatchPacketEventRelayer
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayClientIds
        + CanLog<LevelDebug>
        + CanLog<LevelWarn>
        + CanRelayBatchPackets
        + CanRaiseRelayChainErrors,
    SrcChain: HasErrorType
        + HasSendPacketEvent<DstChain>
        + HasClientIdType<DstChain, ClientId: PartialEq>
        + HasUpdateClientEventFields<DstChain>
        + CanQueryClientStateWithLatestHeight<DstChain>
        + CanExtractFromEvent<SrcChain::SendPacketEvent>
        + CanExtractFromEvent<SrcChain::UpdateClientEvent>
        + CanBuildPacketFromSendPacket<DstChain>
        + CanBuildMisbehaviourMessage<DstChain>
        + CanSendSingleMessage,
    DstChain: CanCheckMisbehaviour<SrcChain>
        + HasEvidenceType
        + HasClientIdType<SrcChain>
        + HasClientStateType<SrcChain>
        + HasErrorType,
    MatchPacketDestinationChain: RelayPacketFilter<Relay>,
{
    async fn relay_chain_batch_events(
        relay: &Relay,
        events: &[EventOf<Relay::SrcChain>],
    ) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        let mut send_packet_events = vec![];

        for event in events.iter() {
            if let Some(send_packet_event) =
                src_chain.try_extract_from_event(PhantomData::<SrcChain::SendPacketEvent>, event)
            {
                let packet = src_chain
                    .build_packet_from_send_packet_event(&send_packet_event)
                    .await
                    .map_err(Relay::raise_error)?;

                if MatchPacketDestinationChain::should_relay_packet(relay, &packet).await? {
                    send_packet_events.push(packet);
                }
            } else if let Some(update_client_event) =
                src_chain.try_extract_from_event(PhantomData::<SrcChain::UpdateClientEvent>, event)
            {
                let src_client_id = src_chain.client_id(&update_client_event);

                // Only process update client events for the client ID that this relay is responsible for
                if &src_client_id != relay.src_client_id() {
                    relay
                        .log(
                            &format!(
                                "Unknown client ID {src_client_id}. Skipping update client event."
                            ),
                            &LevelDebug,
                        )
                        .await;
                    continue;
                }

                let client_state = src_chain
                    .query_client_state_with_latest_height(PhantomData, &src_client_id)
                    .await
                    .map_err(Relay::raise_error)?;

                match dst_chain
                    .check_misbehaviour(&update_client_event, &client_state)
                    .await
                {
                    Ok(Some(evidence)) => {
                        relay
                            .log(
                                "Found misbehaviour, will build message and submit",
                                &LevelDebug,
                            )
                            .await;

                        let msg = src_chain
                            .build_misbehaviour_message(&src_client_id, &evidence)
                            .await
                            .map_err(Relay::raise_error)?;

                        src_chain
                            .send_message(msg)
                            .await
                            .map_err(Relay::raise_error)?;
                    }
                    Ok(None) => {
                        relay.log("no misbehaviour detected", &LevelDebug).await;
                    }
                    Err(e) => {
                        relay
                            .log(
                                &format!("error checking for misbehaviour: {e:?}"),
                                &LevelWarn,
                            )
                            .await;
                    }
                }
            }
        }

        relay.relay_packets(send_packet_events.as_slice()).await?;

        Ok(())
    }
}

#[cgp_provider(BatchEventRelayerComponent)]
impl<Relay, DstChain> BatchEventRelayer<Relay, DestinationTarget> for BatchPacketEventRelayer
where
    Relay: HasRelayChains<DstChain = DstChain>
        + HasRelayClientIds
        + CanRelayBatchAckPackets
        + CanFilterRelayPackets
        + CanRaiseRelayChainErrors,
    DstChain: CanQueryChainHeight
        + CanBuildPacketFromWriteAck<Relay::SrcChain>
        + CanExtractFromEvent<DstChain::WriteAckEvent>,
    MatchPacketSourceChain: RelayPacketFilter<Relay>,
{
    async fn relay_chain_batch_events(
        relay: &Relay,
        events: &[EventOf<Relay::DstChain>],
    ) -> Result<(), Relay::Error> {
        let dst_chain = relay.dst_chain();
        let mut packets_info = vec![];

        for event in events.iter() {
            let m_ack_event = dst_chain.try_extract_from_event(PhantomData, event);

            if let Some(ack_event) = m_ack_event {
                let packet = dst_chain
                    .build_packet_from_write_ack_event(&ack_event)
                    .await
                    .map_err(Relay::raise_error)?;

                /*
                   First check whether the packet is targeted for the destination chain,
                   then use the packet filter in the relay context, as we skip `CanRelayPacket`
                   which would have done the packet filtering.
                */
                if MatchPacketSourceChain::should_relay_packet(relay, &packet).await?
                    && relay.should_relay_packet(&packet).await?
                {
                    let ack = dst_chain
                        .build_ack_from_write_ack_event(&ack_event)
                        .await
                        .map_err(Relay::raise_error)?;

                    packets_info.push((packet, ack))
                }
            }
        }

        let batch_latest_height = dst_chain
            .query_chain_height()
            .await
            .map_err(Relay::raise_error)?;

        relay
            .relay_ack_packets(packets_info.as_slice(), &batch_latest_height)
            .await?;

        Ok(())
    }
}

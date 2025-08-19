use alloc::format;
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
use crate::relay::impls::{
    LogRelayPacketAction, LogSkipRelayLockedPacket, MatchPacketDestinationChain,
    MatchPacketSourceChain, RelayPacketProgress,
};
use crate::relay::traits::{
    CanFilterRelayPackets, CanRaiseRelayChainErrors, CanRelayAckPacket, CanRelayPacket,
    DestinationTarget, EventRelayer, EventRelayerComponent, HasPacketLock, HasRelayChains,
    HasRelayClientIds, RelayPacketFilter, SourceTarget,
};

/**
   A packet event relayer that performs packet relaying based on chain events
   related to IBC packets.

   The implementation of `PacketEventRelayer` for the [`SourceTarget`] is
   different from the [`DestinationTarget`]. This is because the packet
   relaying operations from the source chain is different from the target chain.

   When relaying events from the source chain, the packet event relayer is
   mostly interested in the `SendPacket` event, so that it can relay a
   `RecvPacket` message to the destination chain, or a `TimeoutPacket` message
   to the source chain.

   When relaying events from the destination chain, the packet event relayer
   is mostly interested in the `WriteAck` event, so that it can
   relay a `AckPacket` message to the source chain.
*/
pub struct PacketEventRelayer;

#[cgp_provider(EventRelayerComponent)]
impl<Relay, SrcChain, DstChain> EventRelayer<Relay, SourceTarget> for PacketEventRelayer
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanLog<LevelDebug>
        + CanLog<LevelWarn>
        + CanRelayPacket
        + CanRaiseRelayChainErrors,
    SrcChain: HasErrorType
        + HasSendPacketEvent<DstChain>
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
    async fn relay_chain_event(
        relay: &Relay,
        event: &EventOf<Relay::SrcChain>,
    ) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();
        let dst_chain = relay.dst_chain();

        if let Some(send_packet_event) =
            src_chain.try_extract_from_event(PhantomData::<SrcChain::SendPacketEvent>, event)
        {
            let packet = src_chain
                .build_packet_from_send_packet_event(&send_packet_event)
                .await
                .map_err(Relay::raise_error)?;

            if MatchPacketDestinationChain::should_relay_packet(relay, &packet).await? {
                relay.relay_packet(&packet).await?;
            }
        } else if let Some(update_client_event) =
            src_chain.try_extract_from_event(PhantomData::<SrcChain::UpdateClientEvent>, event)
        {
            let src_client_id = src_chain.client_id(&update_client_event);
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

                    if let Err(e) = src_chain
                        .send_message(msg)
                        .await
                        .map_err(Relay::raise_error)
                    {
                        relay
                            .log(
                                &format!("Failed to submit misbeahviour message: {e:?}"),
                                &LevelWarn,
                            )
                            .await;
                    } else {
                        relay
                        .log(
                            &format!("Successfully submitted misbehaviour message for client {src_client_id}"),
                            &LevelDebug,
                        )
                        .await;
                    }
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

        Ok(())
    }
}

#[cgp_provider(EventRelayerComponent)]
impl<Relay, DstChain> EventRelayer<Relay, DestinationTarget> for PacketEventRelayer
where
    Relay: HasRelayChains<DstChain = DstChain>
        + HasRelayClientIds
        + CanRelayAckPacket
        + CanFilterRelayPackets
        + HasPacketLock
        + for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>
        + for<'a> CanLog<LogRelayPacketAction<'a, Relay>>
        + CanRaiseRelayChainErrors,
    DstChain: CanQueryChainHeight
        + CanBuildPacketFromWriteAck<Relay::SrcChain>
        + CanExtractFromEvent<DstChain::WriteAckEvent>,
    MatchPacketSourceChain: RelayPacketFilter<Relay>,
{
    async fn relay_chain_event(
        relay: &Relay,
        event: &EventOf<Relay::DstChain>,
    ) -> Result<(), Relay::Error> {
        let dst_chain = relay.dst_chain();
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
                let m_lock = relay.try_acquire_packet_lock(&packet).await;

                /*
                   Only relay the ack packet if there isn't another packet relayer
                   trying to relay the same packet. This may happen because packet
                   relayers like `FullCycleRelayer` also relay the ack packet right
                   after it relays the recv packet.

                   On the other hand, this event relayer relays based on the ack
                   event that is fired, which is independent of the main packet
                   relayer. Hence it has to use the packet lock to synchronize
                   with the other packet worker.

                   Note that it is still necessary to handle event-based ack
                   relaying here, as we cannot just rely on the main packet
                   worker to relay the ack packet. It is also possible that the
                   relayer missed the send packet event, which gets relayed by
                   another relayer. In that case, we can still relay the ack
                   packet here based on the ack event.
                */
                match m_lock {
                    Some(_lock) => {
                        let ack = dst_chain
                            .build_ack_from_write_ack_event(&ack_event)
                            .await
                            .map_err(Relay::raise_error)?;

                        let height = dst_chain
                            .query_chain_height()
                            .await
                            .map_err(Relay::raise_error)?;

                        relay
                            .log(
                                "relaying ack packet extracted from ack event",
                                &LogRelayPacketAction {
                                    packet: &packet,
                                    relay_progress: RelayPacketProgress::RelayAckPacket,
                                },
                            )
                            .await;

                        relay.relay_ack_packet(&height, &packet, &ack).await?;

                        relay
                            .log(
                                "successfully relayed ack packet extracted from ack event",
                                &LogRelayPacketAction {
                                    packet: &packet,
                                    relay_progress: RelayPacketProgress::RelayAckPacket,
                                },
                            )
                            .await;
                    }
                    None => {
                        relay.log(
                            "skip relaying ack packet, as another packet relayer has acquired the packet lock",
                            &LogSkipRelayLockedPacket {
                                packet: &packet,
                            }).await;
                    }
                }
            }
        }

        Ok(())
    }
}

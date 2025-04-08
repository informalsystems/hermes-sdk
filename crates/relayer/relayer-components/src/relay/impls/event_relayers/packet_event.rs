use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::extract_data::CanExtractFromEvent;
use hermes_chain_components::traits::packet::from_send_packet::CanBuildPacketFromSendPacket;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::traits::packet::from_write_ack::CanBuildPacketFromWriteAck;
use crate::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use crate::chain::types::aliases::EventOf;
use crate::relay::impls::packet_filters::target::{
    MatchPacketDestinationChain, MatchPacketSourceChain,
};
use crate::relay::impls::packet_relayers::general::full_relay::{
    LogRelayPacketAction, RelayPacketProgress,
};
use crate::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds};
use crate::relay::traits::event_relayer::{EventRelayer, EventRelayerComponent};
use crate::relay::traits::packet_filter::{CanFilterRelayPackets, RelayPacketFilter};
use crate::relay::traits::packet_lock::HasPacketLock;
use crate::relay::traits::packet_relayer::CanRelayPacket;
use crate::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use crate::relay::traits::target::{DestinationTarget, SourceTarget};

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
impl<Relay, SrcChain> EventRelayer<Relay, SourceTarget> for PacketEventRelayer
where
    Relay: HasRelayChains<SrcChain = SrcChain>
        + HasRelayClientIds
        + CanRelayPacket
        + CanRaiseRelayChainErrors,
    SrcChain: HasErrorType
        + HasSendPacketEvent<Relay::DstChain>
        + CanExtractFromEvent<SrcChain::SendPacketEvent>
        + CanBuildPacketFromSendPacket<Relay::DstChain>,
    MatchPacketDestinationChain: RelayPacketFilter<Relay>,
{
    async fn relay_chain_event(
        relay: &Relay,
        event: &EventOf<Relay::SrcChain>,
    ) -> Result<(), Relay::Error> {
        let src_chain = relay.src_chain();

        if let Some(send_packet_event) = src_chain.try_extract_from_event(PhantomData, event) {
            let packet = src_chain
                .build_packet_from_send_packet_event(&send_packet_event)
                .await
                .map_err(Relay::raise_error)?;

            if MatchPacketDestinationChain::should_relay_packet(relay, &packet).await? {
                relay.relay_packet(&packet).await?;
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
               First check whether the packet is targetted for the destination chain,
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
                                    relay,
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
                                    relay,
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
                                relay,
                                packet: &packet,
                            }).await;
                    }
                }
            }
        }

        Ok(())
    }
}

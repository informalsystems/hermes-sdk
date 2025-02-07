use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

use crate::components::default::relay::PacketRelayerComponent;
use crate::relay::traits::chains::{HasRelayChains, PacketOf};
use crate::relay::traits::packet_lock::HasPacketLock;
use crate::relay::traits::packet_relayer::PacketRelayer;

/**
   Call the inner relayer only if the packet lock provided by [`HasPacketLock`]
   is acquired.

   This is to avoid race condition where multiple packet relayers try to
   relay the same packet at the same time.
*/
pub struct LockPacketRelayer<InRelayer>(pub PhantomData<InRelayer>);

pub struct LogSkipRelayLockedPacket<'a, Relay>
where
    Relay: HasRelayChains,
{
    pub relay: &'a Relay,
    pub packet: &'a PacketOf<Relay>,
}

#[cgp_provider(PacketRelayerComponent)]
impl<Relay, InRelayer> PacketRelayer<Relay> for LockPacketRelayer<InRelayer>
where
    Relay: HasRelayChains + HasPacketLock + HasLogger,
    InRelayer: PacketRelayer<Relay>,
    Relay::Logger: for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>,
{
    async fn relay_packet(relay: &Relay, packet: &PacketOf<Relay>) -> Result<(), Relay::Error> {
        let m_lock = relay.try_acquire_packet_lock(packet).await;

        match m_lock {
            Some(_lock) => InRelayer::relay_packet(relay, packet).await,
            None => {
                relay.logger().log(
                    "skip relaying packet, as another packet relayer has acquired the packet lock",
                    &LogSkipRelayLockedPacket {
                        relay,
                        packet,
                    }).await;

                Ok(())
            }
        }
    }
}

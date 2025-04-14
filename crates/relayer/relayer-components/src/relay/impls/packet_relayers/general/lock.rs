use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_logging_components::traits::logger::CanLog;

use crate::relay::traits::{
    HasPacketLock, HasRelayChains, PacketOf, PacketRelayer, PacketRelayerComponent,
};

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
    pub packet: &'a PacketOf<Relay>,
}

#[cgp_provider(PacketRelayerComponent)]
impl<Relay, InRelayer> PacketRelayer<Relay> for LockPacketRelayer<InRelayer>
where
    Relay: HasRelayChains + HasPacketLock + for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>,
    InRelayer: PacketRelayer<Relay>,
{
    async fn relay_packet(relay: &Relay, packet: &PacketOf<Relay>) -> Result<(), Relay::Error> {
        let m_lock = relay.try_acquire_packet_lock(packet).await;

        match m_lock {
            Some(_lock) => InRelayer::relay_packet(relay, packet).await,
            None => {
                relay.log(
                    "skip relaying packet, as another packet relayer has acquired the packet lock",
                    &LogSkipRelayLockedPacket {
                        packet,
                    }).await;

                Ok(())
            }
        }
    }
}

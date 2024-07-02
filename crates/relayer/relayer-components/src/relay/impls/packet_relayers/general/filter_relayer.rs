use core::marker::PhantomData;

use cgp_core::async_trait;

use crate::relay::traits::packet_filter::CanFilterPackets;
use crate::relay::traits::packet_relayer::PacketRelayer;

pub struct FilterRelayer<InRelayer> {
    pub phantom: PhantomData<InRelayer>,
}

impl<Relay, InRelayer> PacketRelayer<Relay> for FilterRelayer<InRelayer>
where
    Relay: CanFilterPackets,
    InRelayer: PacketRelayer<Relay>,
{
    async fn relay_packet(relay: &Relay, packet: &Relay::Packet) -> Result<(), Relay::Error> {
        if relay.should_relay_packet(packet).await? {
            InRelayer::relay_packet(relay, packet).await
        } else {
            Ok(())
        }
    }
}

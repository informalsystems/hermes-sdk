use core::marker::PhantomData;

use hermes_prelude::*;

use crate::relay::traits::{
    CanFilterRelayPackets, PacketOf, PacketRelayer, PacketRelayerComponent,
};

pub struct FilterRelayer<InRelayer> {
    pub phantom: PhantomData<InRelayer>,
}

#[cgp_provider(PacketRelayerComponent)]
impl<Relay, InRelayer> PacketRelayer<Relay> for FilterRelayer<InRelayer>
where
    Relay: CanFilterRelayPackets,
    InRelayer: PacketRelayer<Relay>,
{
    async fn relay_packet(relay: &Relay, packet: &PacketOf<Relay>) -> Result<(), Relay::Error> {
        if relay.should_relay_packet(packet).await? {
            InRelayer::relay_packet(relay, packet).await
        } else {
            Ok(())
        }
    }
}

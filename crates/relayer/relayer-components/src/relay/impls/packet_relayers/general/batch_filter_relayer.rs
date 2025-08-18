use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_prelude::*;

use crate::relay::traits::{
    BatchPacketsRelayer, BatchPacketsRelayerComponent, CanFilterRelayPackets, PacketOf,
};

pub struct BatchFilterRelayer<InRelayer> {
    pub phantom: PhantomData<InRelayer>,
}

#[cgp_provider(BatchPacketsRelayerComponent)]
impl<Relay, InRelayer> BatchPacketsRelayer<Relay> for BatchFilterRelayer<InRelayer>
where
    Relay: CanFilterRelayPackets,
    InRelayer: BatchPacketsRelayer<Relay>,
{
    async fn relay_packets(
        relay: &Relay,
        packets: Vec<&PacketOf<Relay>>,
    ) -> Result<(), Relay::Error> {
        let mut filtered_packets = vec![];
        for packet in packets.iter() {
            if relay.should_relay_packet(packet).await? {
                filtered_packets.push(*packet);
            }
        }
        InRelayer::relay_packets(relay, packets).await
    }
}

use core::marker::PhantomData;

use cgp::prelude::*;

use crate::relay::traits::{
    HasRelayChains, PacketOf, RelayPacketFilter, RelayPacketFilterComponent,
};

pub struct And<FilterA, FilterB>(pub PhantomData<(FilterA, FilterB)>);

#[cgp_provider(RelayPacketFilterComponent)]
impl<Relay, FilterA, FilterB> RelayPacketFilter<Relay> for And<FilterA, FilterB>
where
    Relay: HasRelayChains,
    FilterA: RelayPacketFilter<Relay>,
    FilterB: RelayPacketFilter<Relay>,
{
    async fn should_relay_packet(
        relay: &Relay,
        packet: &PacketOf<Relay>,
    ) -> Result<bool, Relay::Error> {
        if FilterA::should_relay_packet(relay, packet).await? {
            FilterB::should_relay_packet(relay, packet).await
        } else {
            Ok(false)
        }
    }
}

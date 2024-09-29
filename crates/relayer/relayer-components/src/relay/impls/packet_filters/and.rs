use core::marker::PhantomData;

use crate::relay::traits::chains::{HasRelayChains, PacketOf};
use crate::relay::traits::packet_filter::PacketFilter;

pub struct And<FilterA, FilterB>(pub PhantomData<(FilterA, FilterB)>);

impl<Relay, FilterA, FilterB> PacketFilter<Relay> for And<FilterA, FilterB>
where
    Relay: HasRelayChains,
    FilterA: PacketFilter<Relay>,
    FilterB: PacketFilter<Relay>,
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

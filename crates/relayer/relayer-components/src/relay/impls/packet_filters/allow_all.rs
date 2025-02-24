use cgp::prelude::*;

use crate::relay::traits::chains::{HasRelayChains, PacketOf};
use crate::relay::traits::packet_filter::{RelayPacketFilter, RelayPacketFilterComponent};

pub struct AllowAll;

#[cgp_provider(RelayPacketFilterComponent)]
impl<Relay> RelayPacketFilter<Relay> for AllowAll
where
    Relay: HasRelayChains,
{
    async fn should_relay_packet(
        _relay: &Relay,
        _packet: &PacketOf<Relay>,
    ) -> Result<bool, Relay::Error> {
        Ok(true)
    }
}

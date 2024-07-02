use cgp_core::async_trait;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::packet_filter::PacketFilter;

pub struct AllowAll;

impl<Relay> PacketFilter<Relay> for AllowAll
where
    Relay: HasRelayChains,
{
    async fn should_relay_packet(
        _relay: &Relay,
        _packet: &Relay::Packet,
    ) -> Result<bool, Relay::Error> {
        Ok(true)
    }
}

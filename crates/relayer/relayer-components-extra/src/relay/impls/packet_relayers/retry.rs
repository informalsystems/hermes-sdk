use hermes_prelude::*;
use hermes_relayer_components::error::traits::{CanPerformRetry, HasMaxErrorRetry};
use hermes_relayer_components::relay::traits::{
    HasRelayChains, PacketOf, PacketRelayer, PacketRelayerComponent,
};

#[cgp_new_provider(PacketRelayerComponent)]
impl<Relay, InRelayer> PacketRelayer<Relay> for RelayPacketWithRetry<InRelayer>
where
    Relay: HasRelayChains + HasMaxErrorRetry + CanPerformRetry,
    InRelayer: PacketRelayer<Relay>,
{
    async fn relay_packet(relay: &Relay, packet: &PacketOf<Relay>) -> Result<(), Relay::Error> {
        relay
            .perform_with_retry("relay_packet", relay.max_retry(), async || {
                InRelayer::relay_packet(relay, packet).await
            })
            .await
    }
}

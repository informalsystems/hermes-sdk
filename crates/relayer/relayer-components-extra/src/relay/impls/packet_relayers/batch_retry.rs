use hermes_prelude::*;
use hermes_relayer_components::error::traits::{CanPerformRetry, HasMaxErrorRetry};
use hermes_relayer_components::relay::traits::{
    BatchPacketsRelayer, BatchPacketsRelayerComponent, HasRelayChains, PacketOf,
};

#[cgp_new_provider(BatchPacketsRelayerComponent)]
impl<Relay, InRelayer> BatchPacketsRelayer<Relay> for RelayPacketWithRetry<InRelayer>
where
    Relay: HasRelayChains + HasMaxErrorRetry + CanPerformRetry,
    InRelayer: BatchPacketsRelayer<Relay>,
{
    async fn relay_packets(relay: &Relay, packets: &[PacketOf<Relay>]) -> Result<(), Relay::Error> {
        if packets.is_empty() {
            return Ok(());
        }

        relay
            .perform_with_retry("relay_packets", relay.max_retry(), async || {
                InRelayer::relay_packets(relay, packets).await
            })
            .await
    }
}

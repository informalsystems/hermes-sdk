use alloc::format;
use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::{LevelInfo, LevelWarn};
use hermes_prelude::*;

use crate::relay::traits::{
    BatchPacketsRelayer, BatchPacketsRelayerComponent, HasRelayChains, PacketOf,
};

pub struct BatchLoggerRelayer<InRelayer>(pub PhantomData<InRelayer>);

#[cgp_provider(BatchPacketsRelayerComponent)]
impl<Relay, InRelayer> BatchPacketsRelayer<Relay> for BatchLoggerRelayer<InRelayer>
where
    Relay: HasRelayChains + CanLog<LevelWarn> + CanLog<LevelInfo>,
    InRelayer: BatchPacketsRelayer<Relay>,
{
    async fn relay_packets(
        relay: &Relay,
        packets: Vec<&PacketOf<Relay>>,
    ) -> Result<(), Relay::Error> {
        if packets.is_empty() {
            return Ok(());
        }

        let res = InRelayer::relay_packets(relay, packets.clone()).await;

        if let Err(error) = &res {
            relay
                .log(&format!("failed to relay packet: {error:?}"), &LevelWarn)
                .await;
        } else {
            relay
                .log(
                    &format!("successfully relayed packets: {packets:?}"),
                    &LevelInfo,
                )
                .await;
        }

        res
    }
}

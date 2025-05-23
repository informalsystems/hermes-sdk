use core::marker::PhantomData;

use hermes_logging_components::traits::CanLog;
use hermes_prelude::*;

use crate::relay::traits::{HasRelayChains, PacketOf, PacketRelayer, PacketRelayerComponent};

pub struct LoggerRelayer<InRelayer>(pub PhantomData<InRelayer>);

pub struct LogRelayPacketStatus<'a, Relay>
where
    Relay: HasRelayChains,
{
    pub packet: &'a PacketOf<Relay>,
    pub relay_status: RelayPacketStatus<'a, Relay>,
}

pub enum RelayPacketStatus<'a, Relay>
where
    Relay: HasAsyncErrorType,
{
    Start,
    Successful,
    Error { error: &'a Relay::Error },
}

#[cgp_provider(PacketRelayerComponent)]
impl<Relay, InRelayer> PacketRelayer<Relay> for LoggerRelayer<InRelayer>
where
    Relay: HasRelayChains + for<'a> CanLog<LogRelayPacketStatus<'a, Relay>>,
    InRelayer: PacketRelayer<Relay>,
{
    async fn relay_packet(relay: &Relay, packet: &PacketOf<Relay>) -> Result<(), Relay::Error> {
        relay
            .log(
                "starting to relay packet",
                &LogRelayPacketStatus {
                    packet,
                    relay_status: RelayPacketStatus::Start,
                },
            )
            .await;

        let res = InRelayer::relay_packet(relay, packet).await;

        if let Err(error) = &res {
            relay
                .log(
                    "failed to relay packet",
                    &LogRelayPacketStatus {
                        packet,
                        relay_status: RelayPacketStatus::Error { error },
                    },
                )
                .await;
        } else {
            relay
                .log(
                    "successfully relayed packet",
                    &LogRelayPacketStatus {
                        packet,
                        relay_status: RelayPacketStatus::Successful,
                    },
                )
                .await;
        }

        res
    }
}

use core::marker::PhantomData;

use cgp::prelude::HasAsyncErrorType;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

use crate::relay::traits::chains::{HasRelayChains, PacketOf};
use crate::relay::traits::packet_relayer::PacketRelayer;

pub struct LoggerRelayer<InRelayer>(pub PhantomData<InRelayer>);

pub struct LogRelayPacketStatus<'a, Relay>
where
    Relay: HasRelayChains,
{
    pub relay: &'a Relay,
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

impl<Relay, InRelayer> PacketRelayer<Relay> for LoggerRelayer<InRelayer>
where
    Relay: HasRelayChains + HasLogger,
    InRelayer: PacketRelayer<Relay>,
    Relay::Logger: for<'a> CanLog<LogRelayPacketStatus<'a, Relay>>,
{
    async fn relay_packet(relay: &Relay, packet: &PacketOf<Relay>) -> Result<(), Relay::Error> {
        let logger = relay.logger();

        logger
            .log(
                "starting to relay packet",
                &LogRelayPacketStatus {
                    relay,
                    packet,
                    relay_status: RelayPacketStatus::Start,
                },
            )
            .await;

        let res = InRelayer::relay_packet(relay, packet).await;

        if let Err(error) = &res {
            logger
                .log(
                    "failed to relay packet",
                    &LogRelayPacketStatus {
                        relay,
                        packet,
                        relay_status: RelayPacketStatus::Error { error },
                    },
                )
                .await;
        } else {
            logger
                .log(
                    "successfully relayed packet",
                    &LogRelayPacketStatus {
                        relay,
                        packet,
                        relay_status: RelayPacketStatus::Successful,
                    },
                )
                .await;
        }

        res
    }
}

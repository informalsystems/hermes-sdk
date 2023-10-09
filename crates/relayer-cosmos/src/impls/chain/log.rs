use alloc::sync::Arc;
use cgp_core::Async;
use ibc_relayer_components::chain::traits::logs::event::CanLogChainEvent;
use ibc_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use ibc_relayer_components::logger::traits::has_logger::HasLogger;
use ibc_relayer_runtime::types::log::logger::TracingLogger;
use ibc_relayer_runtime::types::log::value::LogValue;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use tendermint::abci::Event as AbciEvent;

use crate::contexts::chain::CosmosChain;

impl<Chain> HasLogger for CosmosChain<Chain>
where
    Chain: Async,
{
    fn logger(&self) -> &TracingLogger {
        &TracingLogger
    }
}

impl<Chain> CanLogChainEvent for CosmosChain<Chain>
where
    Chain: Async,
{
    fn log_event(event: &Arc<AbciEvent>) -> LogValue<'_> {
        LogValue::Debug(event)
    }
}

impl<Chain, Counterparty> CanLogChainPacket<Counterparty> for CosmosChain<Chain>
where
    Chain: Async,
{
    fn log_incoming_packet(packet: &Packet) -> LogValue<'_> {
        LogValue::Display(packet)
    }

    fn log_outgoing_packet(packet: &Packet) -> LogValue<'_> {
        LogValue::Display(packet)
    }
}

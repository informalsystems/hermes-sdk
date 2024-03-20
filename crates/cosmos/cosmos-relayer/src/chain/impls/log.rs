use alloc::sync::Arc;

use hermes_relayer_components::chain::traits::logs::event::CanLogChainEvent;
use hermes_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use hermes_relayer_components::transaction::traits::logs::nonce::CanLogNonce;
use hermes_relayer_runtime::types::log::value::LogValue;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use tendermint::abci::Event as AbciEvent;

use crate::contexts::chain::CosmosChain;

impl CanLogChainEvent for CosmosChain {
    fn log_event(event: &Arc<AbciEvent>) -> LogValue<'_> {
        LogValue::Debug(event)
    }
}

impl<Counterparty> CanLogChainPacket<Counterparty> for CosmosChain {
    fn log_incoming_packet(packet: &Packet) -> LogValue<'_> {
        LogValue::Display(packet)
    }

    fn log_outgoing_packet(packet: &Packet) -> LogValue<'_> {
        LogValue::Display(packet)
    }
}

impl CanLogNonce for CosmosChain {
    fn log_nonce(nonce: &Account) -> LogValue<'_> {
        LogValue::Debug(nonce)
    }
}

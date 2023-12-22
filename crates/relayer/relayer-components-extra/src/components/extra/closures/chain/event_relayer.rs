use cgp_core::{HasComponents, HasErrorType};
use hermes_relayer_components::chain::traits::components::counterparty_chain_id_querier::{
    CanQueryCounterpartyChainId, CounterpartyChainIdQuerier,
};
use hermes_relayer_components::chain::traits::components::packet_from_write_ack_builder::{
    CanBuildPacketFromWriteAck, PacketFromWriteAckBuilder,
};
use hermes_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::logger::traits::has_logger::HasLoggerType;

use crate::components::extra::chain::DelegatesToExtraChainComponents;

pub trait UseExtraChainComponentsForEventRelayer<Counterparty>:
    HasChainId
    + HasLoggerType
    + HasSendPacketEvent<Counterparty>
    + CanLogChainPacket<Counterparty>
    + CanQueryCounterpartyChainId<Counterparty>
    + CanBuildPacketFromWriteAck<Counterparty>
where
    Counterparty: HasIbcChainTypes<Self>,
{
}

impl<Chain, Counterparty, Components, BaseComponents>
    UseExtraChainComponentsForEventRelayer<Counterparty> for Chain
where
    Chain: HasErrorType
        + HasChainId
        + HasLoggerType
        + HasSendPacketEvent<Counterparty>
        + CanLogChainPacket<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasClientStateType<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasComponents<Components = Components>,
    Counterparty: HasIbcChainTypes<Chain>,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToExtraChainComponents<BaseComponents>
        + CounterpartyChainIdQuerier<Chain, Counterparty>
        + PacketFromWriteAckBuilder<Chain, Counterparty>,
{
}

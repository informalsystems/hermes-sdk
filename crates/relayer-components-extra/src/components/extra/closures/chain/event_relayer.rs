use cgp_core::{HasComponents, HasErrorType};
use ibc_relayer_components::chain::traits::components::counterparty_chain_id_querier::{
    CanQueryCounterpartyChainId, CounterpartyChainIdQuerier,
};
use ibc_relayer_components::chain::traits::components::packet_from_write_ack_builder::{
    CanBuildPacketFromWriteAck, PacketFromWriteAckBuilder,
};
use ibc_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use ibc_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use ibc_relayer_components::logger::traits::has_logger::HasLoggerType;

use crate::components::extra::chain::ExtraChainComponents;

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

impl<Chain, Counterparty, ChainComponents> UseExtraChainComponentsForEventRelayer<Counterparty>
    for Chain
where
    Chain: HasErrorType
        + HasChainId
        + HasLoggerType
        + HasSendPacketEvent<Counterparty>
        + CanLogChainPacket<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasClientStateType<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasComponents<Components = ExtraChainComponents<ChainComponents>>,
    Counterparty: HasIbcChainTypes<Chain>,
    ChainComponents: CounterpartyChainIdQuerier<Chain, Counterparty>
        + PacketFromWriteAckBuilder<Chain, Counterparty>,
{
}

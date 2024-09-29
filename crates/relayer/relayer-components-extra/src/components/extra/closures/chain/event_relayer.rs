use cgp::prelude::{HasComponents, HasErrorType};
use hermes_relayer_components::chain::traits::packet::from_write_ack::{
    CanBuildPacketFromWriteAck, PacketFromWriteAckBuilder,
};
use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::{
    CanQueryCounterpartyChainId, CounterpartyChainIdQuerier,
};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;

pub trait UseExtraChainComponentsForEventRelayer<Counterparty>:
    HasChainId
    + HasSendPacketEvent<Counterparty>
    + CanQueryCounterpartyChainId<Counterparty>
    + CanBuildPacketFromWriteAck<Counterparty>
where
    Counterparty: HasIbcChainTypes<Self> + HasOutgoingPacketType<Self>,
{
}

impl<Chain, Counterparty, Components> UseExtraChainComponentsForEventRelayer<Counterparty> for Chain
where
    Chain: HasErrorType
        + HasChainId
        + HasSendPacketEvent<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasClientStateType<Counterparty>
        + HasWriteAckEvent<Counterparty>
        + HasComponents<Components = Components>,
    Counterparty: HasIbcChainTypes<Chain> + HasOutgoingPacketType<Chain>,
    Components: CounterpartyChainIdQuerier<Chain, Counterparty>
        + PacketFromWriteAckBuilder<Chain, Counterparty>,
{
}

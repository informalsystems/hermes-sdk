use cgp_core::HasComponents;
use hermes_relayer_components::chain::traits::components::chain_status_querier::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::components::channel_handshake_message_builder::{
    CanBuildChannelHandshakeMessages, ChannelHandshakeMessageBuilder,
};
use hermes_relayer_components::chain::traits::components::channel_handshake_payload_builder::{
    CanBuildChannelHandshakePayloads, ChannelHandshakePayloadBuilder,
};
use hermes_relayer_components::chain::traits::components::client_state_querier::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloadTypes, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    HasChannelOpenInitEvent, HasChannelOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;

use crate::components::extra::chain::DelegatesToExtraChainComponents;
use crate::components::extra::closures::chain::message_sender::UseExtraChainComponentsForIbcMessageSender;

pub trait UseExtraChainComponentsForChannelHandshake<Counterparty>:
    UseExtraChainComponentsForIbcMessageSender<Counterparty>
    + HasInitChannelOptionsType<Counterparty>
    + HasChannelOpenTryEvent<Counterparty>
    + CanQueryClientState<Counterparty>
    + CanBuildChannelHandshakePayloads<Counterparty>
    + CanBuildChannelHandshakeMessages<Counterparty>
    + HasChannelOpenInitEvent<Counterparty>
    + CanQueryChainHeight
where
    Counterparty: HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasUpdateClientPayload<Self>
        + HasChannelHandshakePayloadTypes<Self>,
{
}

impl<Chain, Counterparty, Components, BaseComponents>
    UseExtraChainComponentsForChannelHandshake<Counterparty> for Chain
where
    Chain: HasChannelOpenInitEvent<Counterparty>
        + HasChannelOpenTryEvent<Counterparty>
        + HasInitChannelOptionsType<Counterparty>
        + HasChannelHandshakePayloadTypes<Counterparty>
        + UseExtraChainComponentsForIbcMessageSender<Counterparty>
        + HasComponents<Components = Components>,
    Counterparty: HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasIbcChainTypes<Chain>
        + HasUpdateClientPayload<Chain>
        + HasChannelHandshakePayloadTypes<Chain>,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToExtraChainComponents<BaseComponents>
        + ChannelHandshakePayloadBuilder<Chain, Counterparty>
        + ChannelHandshakeMessageBuilder<Chain, Counterparty>,
    Chain::Height: Clone,
{
}

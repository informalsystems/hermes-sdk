use cgp_core::HasComponents;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelHandshakeMessages, ChannelHandshakeMessageBuilder,
};
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    CanBuildChannelHandshakePayloads, ChannelHandshakePayloadBuilder,
};
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloadTypes, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    HasChannelOpenInitEvent, HasChannelOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;

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
        + HasUpdateClientPayloadType<Self>
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
        + HasUpdateClientPayloadType<Chain>
        + HasChannelHandshakePayloadTypes<Chain>,
    Components: HasComponents<Components = BaseComponents>
        + DelegatesToExtraChainComponents<BaseComponents>
        + ChannelHandshakePayloadBuilder<Chain, Counterparty>
        + ChannelHandshakeMessageBuilder<Chain, Counterparty>,
    Chain::Height: Clone,
{
}

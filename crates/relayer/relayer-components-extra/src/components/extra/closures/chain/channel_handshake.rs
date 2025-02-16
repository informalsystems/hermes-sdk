use cgp::prelude::HasProvider;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelOpenTryMessage, ChannelOpenTryMessageBuilder,
};
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    CanBuildChannelOpenTryPayload, ChannelOpenTryPayloadBuilder,
};
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelOpenTryPayloadType, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    HasChannelOpenInitEvent, HasChannelOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;

use crate::components::extra::closures::chain::message_sender::UseExtraChainComponentsForIbcMessageSender;

pub trait UseExtraChainComponentsForChannelHandshake<Counterparty>:
    UseExtraChainComponentsForIbcMessageSender<Counterparty>
    + HasInitChannelOptionsType<Counterparty>
    + HasChannelOpenTryEvent<Counterparty>
    + CanQueryClientState<Counterparty>
    + CanBuildChannelOpenTryPayload<Counterparty>
    + CanBuildChannelOpenTryMessage<Counterparty>
    + HasChannelOpenInitEvent<Counterparty>
    + CanQueryChainHeight
where
    Counterparty: HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasIbcChainTypes<Self>
        + HasUpdateClientPayloadType<Self>
        + HasChannelOpenTryPayloadType<Self>,
{
}

impl<Chain, Counterparty, Components> UseExtraChainComponentsForChannelHandshake<Counterparty>
    for Chain
where
    Chain: HasIbcChainTypes<Counterparty>
        + HasChannelOpenInitEvent<Counterparty>
        + HasChannelOpenTryEvent<Counterparty>
        + HasInitChannelOptionsType<Counterparty>
        + HasChannelOpenTryPayloadType<Counterparty>
        + UseExtraChainComponentsForIbcMessageSender<Counterparty>
        + HasProvider<Components = Components>,
    Counterparty: HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasIbcChainTypes<Chain>
        + HasUpdateClientPayloadType<Chain>
        + HasChannelOpenTryPayloadType<Chain>,
    Components: ChannelOpenTryPayloadBuilder<Chain, Counterparty>
        + ChannelOpenTryMessageBuilder<Chain, Counterparty>,
    Chain::Height: Clone,
{
}

use cgp_core::HasComponents;
use ibc_relayer_components::chain::traits::components::chain_status_querier::CanQueryChainHeight;
use ibc_relayer_components::chain::traits::components::channel_handshake_message_builder::{
    CanBuildChannelHandshakeMessages, ChannelHandshakeMessageBuilder,
};
use ibc_relayer_components::chain::traits::components::channel_handshake_payload_builder::{
    CanBuildChannelHandshakePayloads, ChannelHandshakePayloadBuilder,
};
use ibc_relayer_components::chain::traits::components::client_state_querier::CanQueryClientState;
use ibc_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloads, HasInitChannelOptionsType,
};
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateType;
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::ibc_events::channel::{
    HasChannelOpenInitEvent, HasChannelOpenTryEvent,
};
use ibc_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;

use crate::components::extra::chain::ExtraChainComponents;
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
        + HasChannelHandshakePayloads<Self>,
{
}

impl<Chain, Counterparty, ChainComponents> UseExtraChainComponentsForChannelHandshake<Counterparty>
    for Chain
where
    Chain: HasChannelOpenInitEvent<Counterparty>
        + HasChannelOpenTryEvent<Counterparty>
        + HasInitChannelOptionsType<Counterparty>
        + HasChannelHandshakePayloads<Counterparty>
        + UseExtraChainComponentsForIbcMessageSender<Counterparty>
        + HasComponents<Components = ExtraChainComponents<ChainComponents>>,
    Counterparty: HasClientStateType<Chain>
        + HasConsensusStateType<Chain>
        + HasIbcChainTypes<Chain>
        + HasUpdateClientPayload<Chain>
        + HasChannelHandshakePayloads<Chain>,
    ChainComponents: ChannelHandshakePayloadBuilder<Chain, Counterparty>
        + ChannelHandshakeMessageBuilder<Chain, Counterparty>,
    Chain::Height: Clone,
{
}

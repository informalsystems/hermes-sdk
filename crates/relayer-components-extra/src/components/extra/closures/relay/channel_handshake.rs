use cgp_core::HasComponents;
use ibc_relayer_components::chain::traits::components::chain_status_querier::CanQueryChainHeight;
use ibc_relayer_components::chain::traits::components::channel_handshake_message_builder::CanBuildChannelHandshakeMessages;
use ibc_relayer_components::chain::traits::components::channel_handshake_payload_builder::CanBuildChannelHandshakePayloads;
use ibc_relayer_components::chain::traits::components::client_state_querier::CanQueryClientState;
use ibc_relayer_components::chain::traits::components::message_sender::CanSendMessages;
use ibc_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use ibc_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_components::chain::traits::types::ibc_events::channel::{
    HasChannelOpenInitEvent, HasChannelOpenTryEvent,
};
use ibc_relayer_components::relay::impls::channel::open_init::CanRaiseMissingChannelInitEventError;
use ibc_relayer_components::relay::impls::channel::open_try::CanRaiseMissingChannelTryEventError;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::relay::traits::channel::open_ack::ChannelOpenAckRelayer;
use ibc_relayer_components::relay::traits::channel::open_confirm::ChannelOpenConfirmRelayer;
use ibc_relayer_components::relay::traits::channel::open_handshake::CanRelayChannelOpenHandshake;
use ibc_relayer_components::relay::traits::channel::open_init::{
    CanInitChannel, ChannelInitializer,
};
use ibc_relayer_components::relay::traits::channel::open_try::ChannelOpenTryRelayer;

use crate::components::extra::closures::relay::message_sender::UseExtraIbcMessageSender;
use crate::components::extra::relay::ExtraRelayComponents;

pub trait UseExtraChannelHandshakeRelayer: CanInitChannel + CanRelayChannelOpenHandshake
where
    Self::SrcChain: HasInitChannelOptionsType<Self::DstChain>,
{
}

impl<Relay, SrcChain, DstChain, RelayComponents> UseExtraChannelHandshakeRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasComponents<Components = ExtraRelayComponents<RelayComponents>>
        + CanRaiseMissingChannelInitEventError
        + CanRaiseMissingChannelTryEventError
        + UseExtraIbcMessageSender,
    RelayComponents: ChannelOpenTryRelayer<Relay>
        + ChannelOpenAckRelayer<Relay>
        + ChannelOpenConfirmRelayer<Relay>
        + ChannelInitializer<Relay>,
    SrcChain: HasIbcChainTypes<DstChain>
        + HasInitChannelOptionsType<DstChain>
        + CanSendMessages
        + CanQueryClientState<DstChain>
        + CanBuildChannelHandshakePayloads<DstChain>
        + CanBuildChannelHandshakeMessages<DstChain>
        + HasChannelOpenInitEvent<DstChain>
        + CanQueryChainHeight,
    DstChain: HasIbcChainTypes<SrcChain>
        + CanQueryClientState<SrcChain>
        + HasChannelOpenTryEvent<SrcChain>
        + CanBuildChannelHandshakePayloads<SrcChain>
        + CanBuildChannelHandshakeMessages<SrcChain>
        + CanQueryChainHeight,
    SrcChain::ChannelId: Clone,
    DstChain::ChannelId: Clone,
{
}

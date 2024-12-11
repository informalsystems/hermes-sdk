use cgp::prelude::{CanRaiseError, HasComponents};
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::relay::impls::channel::open_init::MissingChannelInitEventError;
use hermes_relayer_components::relay::impls::channel::open_try::MissingChannelTryEventError;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::channel::open_ack::ChannelOpenAckRelayer;
use hermes_relayer_components::relay::traits::channel::open_confirm::ChannelOpenConfirmRelayer;
use hermes_relayer_components::relay::traits::channel::open_handshake::CanRelayChannelOpenHandshake;
use hermes_relayer_components::relay::traits::channel::open_init::{
    CanInitChannel, ChannelInitializer,
};
use hermes_relayer_components::relay::traits::channel::open_try::ChannelOpenTryRelayer;
use hermes_relayer_components::relay::traits::target::{
    HasDestinationTargetChainTypes, HasSourceTargetChainTypes,
};

use crate::components::extra::closures::chain::channel_handshake::UseExtraChainComponentsForChannelHandshake;
use crate::components::extra::closures::relay::message_sender::UseExtraIbcMessageSender;
use crate::components::extra::relay::DelegatesToExtraRelayPreset;

pub trait UseExtraChannelHandshakeRelayer: CanInitChannel + CanRelayChannelOpenHandshake
where
    Self::SrcChain: HasInitChannelOptionsType<Self::DstChain>,
{
}

impl<Relay, SrcChain, DstChain, Components> UseExtraChannelHandshakeRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasComponents<Components = Components>
        + CanRaiseError<SrcChain::Error>
        + CanRaiseError<DstChain::Error>
        + for<'a> CanRaiseError<MissingChannelInitEventError<'a, Relay>>
        + for<'a> CanRaiseError<MissingChannelTryEventError<'a, Relay>>
        + UseExtraIbcMessageSender,
    Components: DelegatesToExtraRelayPreset
        + ChannelOpenTryRelayer<Relay>
        + ChannelOpenAckRelayer<Relay>
        + ChannelOpenConfirmRelayer<Relay>
        + ChannelInitializer<Relay>,
    SrcChain: UseExtraChainComponentsForChannelHandshake<DstChain>,
    DstChain: UseExtraChainComponentsForChannelHandshake<SrcChain>,
    SrcChain::ChannelId: Clone,
    DstChain::ChannelId: Clone,
{
}

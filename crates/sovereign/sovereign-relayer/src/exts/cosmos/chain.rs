use cgp_core::prelude::*;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelOpenAckMessage, CanBuildChannelOpenConfirmMessage,
    CanBuildChannelOpenInitMessage, CanBuildChannelOpenTryMessage,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::CanBuildReceivePacketMessage;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::CanBuildTimeoutUnorderedPacketMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    CanBuildChannelOpenAckPayload, CanBuildChannelOpenConfirmPayload, CanBuildChannelOpenTryPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmPayload,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::CanBuildReceivePacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::CanBuildTimeoutUnorderedPacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryClientState, CanQueryClientStateWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::{
    CanQueryConsensusStateHeight, CanQueryConsensusStateHeights,
};
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use hermes_sovereign_chain_components::cosmos::components::SovereignCosmosComponents;

use crate::contexts::sovereign_chain::SovereignChain;

delegate_components! {
    DelegateCosmosChainComponents {
        SovereignChain: SovereignCosmosComponents,
    }
}

pub trait CanUseCosmosChainWithSovereign:
    CanQueryClientState<SovereignChain>
    + CanQueryClientStateWithProofs<SovereignChain>
    + CanQueryConsensusState<SovereignChain>
    + CanQueryConsensusStateWithProofs<SovereignChain>
    + CanQueryConsensusStateHeight<SovereignChain>
    + CanQueryConsensusStateHeights<SovereignChain>
    + CanBuildCreateClientMessage<SovereignChain>
    + CanBuildUpdateClientMessage<SovereignChain>
    + CanBuildConnectionOpenInitMessage<SovereignChain>
    + CanBuildConnectionOpenTryMessage<SovereignChain>
    + CanBuildConnectionOpenAckMessage<SovereignChain>
    + CanBuildConnectionOpenConfirmMessage<SovereignChain>
    + CanBuildConnectionOpenInitPayload<SovereignChain>
    + CanBuildConnectionOpenTryPayload<SovereignChain>
    + CanBuildConnectionOpenAckPayload<SovereignChain>
    + CanBuildConnectionOpenConfirmPayload<SovereignChain>
    + CanBuildChannelOpenTryPayload<SovereignChain>
    + CanBuildChannelOpenAckPayload<SovereignChain>
    + CanBuildChannelOpenConfirmPayload<SovereignChain>
    + CanBuildChannelOpenInitMessage<SovereignChain>
    + CanBuildChannelOpenTryMessage<SovereignChain>
    + CanBuildChannelOpenAckMessage<SovereignChain>
    + CanBuildChannelOpenConfirmMessage<SovereignChain>
    + CanBuildReceivePacketPayload<SovereignChain>
    + CanBuildAckPacketPayload<SovereignChain>
    + CanBuildTimeoutUnorderedPacketPayload<SovereignChain>
    + CanBuildReceivePacketMessage<SovereignChain>
    + CanBuildAckPacketMessage<SovereignChain>
    + CanBuildTimeoutUnorderedPacketMessage<SovereignChain>
    + HasCreateClientPayloadOptionsType<SovereignChain>
    + CanBuildCreateClientPayload<SovereignChain>
    + CanBuildUpdateClientPayload<SovereignChain>
    + HasClientStateFields<SovereignChain>
    + HasInitChannelOptionsType<SovereignChain>
    + HasCounterpartyMessageHeight<SovereignChain>
{
}

impl CanUseCosmosChainWithSovereign for CosmosChain {}

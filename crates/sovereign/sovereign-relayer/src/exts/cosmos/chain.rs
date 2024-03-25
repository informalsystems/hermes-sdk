use cgp_core::prelude::*;
use hermes_cosmos_relayer::chain::impls::connection_handshake_message::DelegateCosmosConnectionHandshakeBuilder;
use hermes_cosmos_relayer::chain::impls::create_client_message::DelegateCosmosCreateClientMessageBuilder;
use hermes_cosmos_relayer::chain::impls::update_client_message::DelegateCosmosUpdateClientMessageBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::client::create_client_message::BuildCreateSovereignClientMessageOnCosmos;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::client::update_client_message::BuildUpdateSovereignClientMessageOnCosmos;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::connection_handshake_message::BuildSovereignConnectionHandshakeMessageOnCosmos;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::CanBuildConnectionHandshakeMessages;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;

use crate::contexts::sovereign_chain::SovereignChain;

pub trait CanUseSovereignMethodsOnCosmosChain:
    CanQueryClientState<SovereignChain>
    + CanBuildCreateClientMessage<SovereignChain>
    + CanBuildUpdateClientMessage<SovereignChain>
    + CanBuildConnectionHandshakeMessages<SovereignChain>
{
}

impl CanUseSovereignMethodsOnCosmosChain for CosmosChain {}

delegate_components! {
    DelegateCosmosCreateClientMessageBuilder {
        SovereignChain: BuildCreateSovereignClientMessageOnCosmos,
    }
}

delegate_components! {
    DelegateCosmosUpdateClientMessageBuilder {
        SovereignChain: BuildUpdateSovereignClientMessageOnCosmos,
    }
}

delegate_components! {
    DelegateCosmosConnectionHandshakeBuilder {
        SovereignChain: BuildSovereignConnectionHandshakeMessageOnCosmos,
    }
}

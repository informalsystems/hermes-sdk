use cgp_core::prelude::*;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientOptionsType;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::client::create_client_message::BuildCreateSovereignClientMessageOnCosmos;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::client::update_client_message::BuildUpdateSovereignClientMessageOnCosmos;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::connection_handshake_message::BuildSovereignConnectionHandshakeMessageOnCosmos;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{CanBuildConnectionHandshakeMessages, ConnectionHandshakeMessageBuilderComponent};
use hermes_relayer_components::chain::traits::message_builders::create_client::{CanBuildCreateClientMessage, CreateClientMessageBuilderComponent};
use hermes_relayer_components::chain::traits::message_builders::update_client::{CanBuildUpdateClientMessage, UpdateClientMessageBuilderComponent};
use hermes_relayer_components::chain::traits::queries::client_state::{CanQueryClientState, ClientStateQuerierComponent};

use crate::contexts::sovereign_chain::SovereignChain;

pub struct SovereignCosmosComponents;

delegate_components! {
    DelegateCosmosChainComponents {
        SovereignChain: SovereignCosmosComponents,
    }
}

delegate_components! {
    SovereignCosmosComponents {
        [
            ClientStateQuerierComponent,
        ]:
            QueryAndConvertRawClientState,
        UpdateClientMessageBuilderComponent:
            BuildUpdateSovereignClientMessageOnCosmos,
        CreateClientMessageBuilderComponent:
            BuildCreateSovereignClientMessageOnCosmos,
        ConnectionHandshakeMessageBuilderComponent:
            BuildSovereignConnectionHandshakeMessageOnCosmos,
    }
}

pub trait CanUseCosmosChainWithSovereign:
    CanQueryClientState<SovereignChain>
    + CanBuildCreateClientMessage<SovereignChain>
    + CanBuildUpdateClientMessage<SovereignChain>
    + CanBuildConnectionHandshakeMessages<SovereignChain>
    + HasCreateClientOptionsType<SovereignChain>
    + CanBuildCreateClientPayload<SovereignChain>
    + CanBuildUpdateClientPayload<SovereignChain>
    + HasClientStateFields<SovereignChain>
{
}

impl CanUseCosmosChainWithSovereign for CosmosChain {}

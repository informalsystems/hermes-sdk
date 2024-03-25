use cgp_core::prelude::*;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_relayer::chain::impls::connection_handshake_message::DelegateCosmosConnectionHandshakeBuilder;
use hermes_cosmos_relayer::chain::impls::update_client_message::DelegateCosmosUpdateClientMessageBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::impls::delegate::queries::client_state::QueryAndDecodeClientStateVia;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::client::create_client_message::BuildCreateSovereignClientMessageOnCosmos;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::client::update_client_message::BuildUpdateSovereignClientMessageOnCosmos;
use hermes_sovereign_chain_components::cosmos::impls::sovereign_to_cosmos::connection_handshake_message::BuildSovereignConnectionHandshakeMessageOnCosmos;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::CanBuildConnectionHandshakeMessages;
use hermes_relayer_components::chain::traits::message_builders::create_client::{CanBuildCreateClientMessage, CreateClientMessageBuilderComponent};
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::queries::client_state::{AllClientStatesBytesQuerierComponent, CanQueryClientState, ClientStateQuerierComponent};
use hermes_wasm_client_components::types::client_state::WasmClientState;

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
            AllClientStatesBytesQuerierComponent,
        ]:
            QueryAndDecodeClientStateVia<WasmClientState>,
        CreateClientMessageBuilderComponent:
            BuildCreateSovereignClientMessageOnCosmos,
    }
}

pub trait CanUseSovereignMethodsOnCosmosChain:
    CanQueryClientState<SovereignChain>
    + CanBuildCreateClientMessage<SovereignChain>
    + CanBuildUpdateClientMessage<SovereignChain>
    + CanBuildConnectionHandshakeMessages<SovereignChain>
{
}

impl CanUseSovereignMethodsOnCosmosChain for CosmosChain {}

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

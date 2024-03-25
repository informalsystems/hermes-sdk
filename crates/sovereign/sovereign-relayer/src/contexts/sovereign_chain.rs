use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_chain_components::impls::queries::client_state::CosmosQueryClientStateComponents;
use hermes_cosmos_relayer::chain::impls::connection_handshake_message::DelegateCosmosConnectionHandshakeBuilder;
use hermes_cosmos_relayer::chain::impls::create_client_message::DelegateCosmosCreateClientMessageBuilder;
use hermes_cosmos_relayer::chain::impls::update_client_message::DelegateCosmosUpdateClientMessageBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent, HasEncoding,
};
use hermes_relayer_components::chain::impls::queries::client_state::QueryAndDecodeClientStateVia;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::CanBuildConnectionHandshakeMessages;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::CanBuildConnectionHandshakePayloads;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use hermes_sovereign_chain_components::cosmos::impls::client::create_client_message::BuildCreateSovereignClientMessageOnCosmos;
use hermes_sovereign_chain_components::cosmos::impls::client::update_client_message::BuildUpdateSovereignClientMessageOnCosmos;
use hermes_sovereign_chain_components::cosmos::impls::connection_handshake_message::BuildSovereignConnectionHandshakeMessageOnCosmos;
use hermes_sovereign_chain_components::sovereign::components::chain::{
    IsSovereignChainClientComponent, SovereignChainClientComponents,
};
use hermes_sovereign_chain_components::sovereign::traits::chain::data_chain::{
    DataChainGetter, DataChainGetterComponent, DataChainTypeComponent, HasDataChain,
    ProvideDataChainType,
};
use hermes_sovereign_chain_components::sovereign::types::client_state::SovereignClientState;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_wasm_client_components::types::client_state::WasmClientState;

use crate::contexts::encoding::{ProvideSovereignEncoding, SovereignEncoding};

pub struct SovereignChain {
    pub runtime: HermesRuntime,
    pub data_chain: CosmosChain,
    // TODO: fields such as rollup JSON RPC address
}

pub struct SovereignChainComponents;

impl HasComponents for SovereignChain {
    type Components = SovereignChainComponents;
}

delegate_all!(
    IsSovereignChainClientComponent,
    SovereignChainClientComponents,
    SovereignChainComponents,
);

pub struct SovereignDataChainType;

impl<Chain> ProvideDataChainType<Chain> for SovereignDataChainType
where
    Chain: Async,
{
    type DataChain = CosmosChain;
}

impl DataChainGetter<SovereignChain> for SovereignDataChainType {
    fn data_chain(chain: &SovereignChain) -> &CosmosChain {
        &chain.data_chain
    }
}

delegate_components! {
    SovereignChainComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
        [
            DataChainTypeComponent,
            DataChainGetterComponent,
        ]: SovereignDataChainType,
        [
            EncodingTypeComponent,
            DefaultEncodingGetterComponent,
        ]:
            ProvideSovereignEncoding,
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

delegate_components! {
    CosmosQueryClientStateComponents {
        SovereignChain: QueryAndDecodeClientStateVia<WasmClientState>,
    }
}

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

impl RuntimeGetter<SovereignChain> for SovereignChainComponents {
    fn runtime(chain: &SovereignChain) -> &HermesRuntime {
        &chain.runtime
    }
}

pub trait CheckSovereignChainImpls:
    HasDataChain
    + HasUpdateClientPayloadType<CosmosChain>
    + HasHeightType<Height = RollupHeight>
    + HasClientStateType<CosmosChain, ClientState = SovereignClientState>
    + CanBuildUpdateClientPayload<CosmosChain>
    + HasEncoding<Encoding = SovereignEncoding>
{
}

impl CheckSovereignChainImpls for SovereignChain {}

pub trait CheckCosmosChainImpls:
    CanQueryClientState<SovereignChain>
    + CanBuildCreateClientMessage<SovereignChain>
    + CanBuildUpdateClientMessage<SovereignChain>
    + CanBuildConnectionHandshakeMessages<SovereignChain>
{
}

impl CheckCosmosChainImpls for CosmosChain {}

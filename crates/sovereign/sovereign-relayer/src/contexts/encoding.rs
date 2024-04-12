use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, HasEncodingType, ProvideEncodingType,
};
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::types::Any;
use hermes_sovereign_chain_components::encoding::components::{
    IsSovereignEncodingComponent, SovereignEncodingComponents as BaseSovereignEncodingComponents,
};
use hermes_sovereign_chain_components::sovereign::types::client_state::SovereignClientState;
use hermes_sovereign_chain_components::sovereign::types::consensus_state::SovereignConsensusState;
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use hermes_wasm_client_components::types::consensus_state::WasmConsensusState;

pub struct SovereignEncoding;

pub struct SovereignEncodingComponents;

impl HasComponents for SovereignEncoding {
    type Components = SovereignEncodingComponents;
}

delegate_all!(
    IsSovereignEncodingComponent,
    BaseSovereignEncodingComponents,
    SovereignEncodingComponents
);

delegate_components! {
    SovereignEncodingComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
    }
}

pub struct ProvideSovereignEncoding;

impl<Context> ProvideEncodingType<Context> for ProvideSovereignEncoding
where
    Context: Async,
{
    type Encoding = SovereignEncoding;
}

impl<Context> DefaultEncodingGetter<Context> for ProvideSovereignEncoding
where
    Context: HasEncodingType<Encoding = SovereignEncoding>,
{
    fn default_encoding() -> &'static SovereignEncoding {
        &SovereignEncoding
    }
}

pub trait CanUseSovereignEncoding:
    CanDecode<ProtoWasmClientState>
    + CanEncodeAndDecode<WasmClientState>
    + CanEncodeAndDecode<Via<Any, WasmClientState>>
    + CanEncodeAndDecode<Via<Any, WasmConsensusState>>
    + CanEncodeAndDecode<SovereignClientState>
    + CanEncodeAndDecode<SovereignConsensusState>
    + CanEncodeAndDecode<Via<Any, SovereignClientState>>
    + CanEncodeAndDecode<Via<Any, SovereignConsensusState>>
    + CanDecode<Via<WasmClientState, SovereignClientState>>
    + CanEncodeAndDecode<Via<WasmConsensusState, SovereignConsensusState>>
    + CanConvertBothWays<WasmClientState, Any>
    + CanConvertBothWays<WasmConsensusState, Any>
{
}

impl CanUseSovereignEncoding for SovereignEncoding {}

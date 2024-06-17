use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvertBothWays;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encode_and_decode::CanEncodeAndDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasEncodingType, ProvideEncodingType,
};
use hermes_protobuf_encoding_components::types::{Any, Protobuf};
use hermes_sovereign_chain_components::encoding::components::{
    IsSovereignEncodingComponent, SovereignEncodingComponents as BaseSovereignEncodingComponents,
};
use hermes_sovereign_rollup_components::types::client_state::{
    SovereignClientState, WrappedSovereignClientState,
};
use hermes_sovereign_rollup_components::types::consensus_state::SovereignConsensusState;
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

delegate_components! {
    ProvideSovereignEncoding {
        EncodingGetterComponent: GetDefaultEncoding,
    }
}

pub trait CanUseSovereignEncoding:
    CanEncodeAndDecode<Protobuf, ProtoWasmClientState>
    + CanEncodeAndDecode<Protobuf, WasmClientState>
    + CanEncodeAndDecode<Any, WasmClientState>
    + CanEncodeAndDecode<Any, WasmConsensusState>
    + CanEncodeAndDecode<Protobuf, SovereignClientState>
    + CanEncodeAndDecode<Protobuf, SovereignConsensusState>
    + CanEncodeAndDecode<Any, SovereignClientState>
    + CanEncodeAndDecode<Any, SovereignConsensusState>
    + CanDecode<WasmClientState, SovereignClientState>
    + CanEncodeAndDecode<WasmConsensusState, SovereignConsensusState>
    + CanConvertBothWays<WasmClientState, Any>
    + CanConvertBothWays<WasmConsensusState, Any>
    + CanEncodeAndDecode<Protobuf, TendermintClientState>
    + CanEncodeAndDecode<Any, TendermintClientState>
    + CanEncodeAndDecode<Protobuf, TendermintConsensusState>
    + CanEncodeAndDecode<Any, TendermintConsensusState>
    + CanConvertBothWays<Any, TendermintClientState>
    + CanConvertBothWays<Any, TendermintConsensusState>
    + CanConvertBothWays<Any, WrappedSovereignClientState>
    + CanConvertBothWays<Any, SovereignConsensusState>
{
}

impl CanUseSovereignEncoding for SovereignEncoding {}

use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_protobuf_components::traits::encoding::HasProtobufEncoding;
use hermes_protobuf_components::traits::encoding::ProtobufEncodingGetter;
use hermes_protobuf_components::traits::encoding::ProvideProtobufEncodingType;
use hermes_protobuf_components::types::Any;
use hermes_relayer_components::encode::traits::decoder::CanDecode;
use hermes_relayer_components::encode::types::wrap::Wrap;
use hermes_sovereign_client_components::encoding::components::IsSovereignEncodingComponent;
use hermes_sovereign_client_components::encoding::components::SovereignEncodingComponents as BaseSovereignEncodingComponents;
use hermes_sovereign_client_components::sovereign::types::client_state::SovereignClientState;
use hermes_wasm_client_components::types::client_state::ProtoWasmClientState;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use ibc_proto_new::ibc::lightclients::sovereign::tendermint::v1::ClientState as ProtoSovereignClientState;

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

impl<Context> ProvideProtobufEncodingType<Context> for ProvideSovereignEncoding
where
    Context: Async,
{
    type Encoding = SovereignEncoding;
}

impl<Context> ProtobufEncodingGetter<Context> for ProvideSovereignEncoding
where
    Context: HasProtobufEncoding<Encoding = SovereignEncoding>,
{
    fn encoding(_context: &Context) -> &SovereignEncoding {
        &SovereignEncoding
    }
}

pub trait CanUseSovereignEncoding:
    CanDecode<ProtoWasmClientState>
    + CanDecode<WasmClientState>
    + CanDecode<Wrap<Any, WasmClientState>>
    + CanDecode<ProtoSovereignClientState>
    + CanDecode<SovereignClientState>
    + CanDecode<Wrap<WasmClientState, SovereignClientState>>
{
}

impl CanUseSovereignEncoding for SovereignEncoding {}

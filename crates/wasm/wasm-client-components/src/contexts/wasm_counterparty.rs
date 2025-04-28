use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use hermes_core::encoding_components::impls::GetDefaultEncoding;
use hermes_core::encoding_components::traits::{
    CanConvert, CanDecode, DefaultEncodingGetter, DefaultEncodingGetterComponent,
    EncodingGetterComponent, EncodingTypeProviderComponent,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::relayer_components::chain::impls::QueryAndConvertRawClientState;
use hermes_core::relayer_components::chain::traits::{
    ChainIdTypeProviderComponent, ChainStatusTypeComponent, ChannelIdTypeComponent,
    ClientIdTypeComponent, ClientStateQuerierComponent, ClientStateTypeComponent,
    ConnectionIdTypeComponent, HeightTypeProviderComponent, OutgoingPacketTypeComponent,
    PortIdTypeComponent, SequenceTypeComponent, TimeoutTypeComponent,
};
use hermes_cosmos_chain_components::impls::ProvideCosmosChainTypes;
use hermes_cosmos_chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_error::handlers::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_prelude::*;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_wasm_encoding_components::components::*;
use hermes_wasm_encoding_components::types::WasmClientState;

use crate::impls::ProvideWasmClientState;

#[cgp_context(WasmCounterpartyComponents)]
pub struct WasmCounterparty;

delegate_components! {
    WasmCounterpartyComponents {
        EncodingTypeProviderComponent<AsBytes>:
            UseType<WasmClientEncoding>,
        [
            HeightTypeProviderComponent,
            TimeoutTypeComponent,
            ChainIdTypeProviderComponent,
            ClientIdTypeComponent,
            ConnectionIdTypeComponent,
            ChannelIdTypeComponent,
            PortIdTypeComponent,
            SequenceTypeComponent,
            OutgoingPacketTypeComponent,
            ChainStatusTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        ClientStateTypeComponent:
            ProvideWasmClientState,
        EncodingGetterComponent<AsBytes>:
            GetDefaultEncoding,
    }
}

pub struct WasmCounterpartyCosmosComponents;

delegate_components! {
    WasmCounterpartyCosmosComponents {
        ClientStateQuerierComponent: QueryAndConvertRawClientState,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        WasmCounterparty: WasmCounterpartyCosmosComponents,
    }
}

#[cgp_provider(DefaultEncodingGetterComponent<AsBytes>)]
impl DefaultEncodingGetter<WasmCounterparty, AsBytes> for WasmCounterpartyComponents {
    fn default_encoding() -> &'static WasmClientEncoding {
        &WasmClientEncoding
    }
}

#[cgp_context(WasmClientEncodingComponents: WasmEncodingComponents)]
pub struct WasmClientEncoding;

delegate_components! {
    WasmClientEncodingComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
    }
}

pub trait CanUseWasmClientEncoding:
    CanDecode<ViaProtobuf, WasmClientState>
    + CanDecode<ViaAny, WasmClientState>
    + CanConvert<Any, WasmClientState>
{
}

impl CanUseWasmClientEncoding for WasmClientEncoding {}

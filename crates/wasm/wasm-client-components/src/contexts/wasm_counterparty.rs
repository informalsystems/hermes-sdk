use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_cosmos_chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetterComponent,
    EncodingTypeProviderComponent,
};
use hermes_encoding_components::types::AsBytes;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerierComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::ibc::{
    ChannelIdTypeComponent, ClientIdTypeComponent, ConnectionIdTypeComponent, PortIdTypeComponent,
    SequenceTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packet::OutgoingPacketTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimeoutTypeComponent;
use hermes_wasm_encoding_components::components::*;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;

use crate::impls::types::client_state::ProvideWasmClientState;

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

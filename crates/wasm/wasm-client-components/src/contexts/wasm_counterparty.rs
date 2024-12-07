use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::{
    ChannelIdTypeComponent, ClientIdTypeComponent, ConnectionIdTypeComponent,
    OutgoingPacketTypeComponent, PortIdTypeComponent, SequenceTypeComponent,
};
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, ProvideEncodingType,
};
use hermes_encoding_components::types::AsBytes;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerierComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimeoutTypeComponent;
use hermes_wasm_encoding_components::components::*;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;

use crate::impls::types::client_state::ProvideWasmClientState;

pub struct WasmCounterparty;

pub struct WasmCounterpartyComponents;

impl HasComponents for WasmCounterparty {
    type Components = WasmCounterpartyComponents;
}

delegate_components! {
    WasmCounterpartyComponents {
        [
            HeightTypeComponent,
            TimeoutTypeComponent,
            ChainIdTypeComponent,
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
        EncodingGetterComponent:
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

impl ProvideEncodingType<WasmCounterparty, AsBytes> for WasmCounterpartyComponents {
    type Encoding = WasmClientEncoding;
}

impl DefaultEncodingGetter<WasmCounterparty, AsBytes> for WasmCounterpartyComponents {
    fn default_encoding() -> &'static WasmClientEncoding {
        &WasmClientEncoding
    }
}

pub struct WasmClientEncoding;

pub struct WasmClientEncodingComponents;

impl HasComponents for WasmClientEncoding {
    type Components = WasmClientEncodingComponents;
}

with_wasm_encoding_components! {
    delegate_components! {
        WasmClientEncodingComponents {
            @WasmEncodingComponents: WasmEncodingComponents,
        }
    }
}

delegate_components! {
    WasmClientEncodingComponents {
        ErrorTypeComponent: ProvideHermesError,
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

use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, ProvideEncodingType,
};
use hermes_protobuf_encoding_components::types::{Any, Protobuf};
use hermes_relayer_components::chain::impls::queries::query_and_decode_client_state::QueryAndDecodeClientState;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesBytesQuerierComponent, ClientStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::impls::encoding::components::{IsWasmEncodingComponent, WasmEncodingComponents};
use crate::impls::types::client_state::ProvideWasmClientState;
use crate::types::client_state::{ProtoWasmClientState, WasmClientState};

pub struct WasmCounterparty;

pub struct WasmCounterpartyComponents;

impl HasComponents for WasmCounterparty {
    type Components = WasmCounterpartyComponents;
}

delegate_components! {
    WasmCounterpartyComponents {
        [
            HeightTypeComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
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
        [
            ClientStateQuerierComponent,
            AllClientStatesBytesQuerierComponent,
        ]: QueryAndDecodeClientState<Any>,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        WasmCounterparty: WasmCounterpartyCosmosComponents,
    }
}

impl ProvideEncodingType<WasmCounterparty> for WasmCounterpartyComponents {
    type Encoding = WasmClientEncoding;
}

impl DefaultEncodingGetter<WasmCounterparty> for WasmCounterpartyComponents {
    fn default_encoding() -> &'static WasmClientEncoding {
        &WasmClientEncoding
    }
}

pub struct WasmClientEncoding;

pub struct WasmClientEncodingComponents;

impl HasComponents for WasmClientEncoding {
    type Components = WasmClientEncodingComponents;
}

delegate_all!(
    IsWasmEncodingComponent,
    WasmEncodingComponents,
    WasmClientEncodingComponents,
);

delegate_components! {
    WasmClientEncodingComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
    }
}

pub trait CanUseWasmClientEncoding:
    CanDecode<Protobuf, ProtoWasmClientState>
    + CanDecode<Protobuf, WasmClientState>
    + CanDecode<Any, WasmClientState>
{
}

impl CanUseWasmClientEncoding for WasmClientEncoding {}

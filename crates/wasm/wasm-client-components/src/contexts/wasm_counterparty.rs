use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_client_components::impls::queries::client_state::CosmosQueryClientStateComponents;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_protobuf_components::types::Any;
use hermes_relayer_components::chain::impls::queries::client_state::QueryAndDecodeClientStateVia;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::encode::traits::decoder::CanDecode;
use hermes_relayer_components::encode::traits::has_encoding::EncodingGetter;
use hermes_relayer_components::encode::traits::has_encoding::ProvideEncodingType;
use hermes_relayer_components::encode::types::via::Via;

use crate::impls::encoding::components::IsWasmEncodingComponent;
use crate::impls::encoding::components::WasmEncodingComponents;
use crate::impls::types::client_state::ProvideWasmClientState;
use crate::types::client_state::ProtoWasmClientState;
use crate::types::client_state::WasmClientState;

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
    }
}

delegate_components! {
    CosmosQueryClientStateComponents {
        WasmCounterparty: QueryAndDecodeClientStateVia<Any>
    }
}

impl ProvideEncodingType<WasmCounterparty> for WasmCounterpartyComponents {
    type Encoding = WasmClientEncoding;
}

impl EncodingGetter<WasmCounterparty> for WasmCounterpartyComponents {
    fn encoding(_context: &WasmCounterparty) -> &WasmClientEncoding {
        &WasmClientEncoding
    }
}

#[derive(Default)]
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
    CanDecode<ProtoWasmClientState> + CanDecode<WasmClientState> + CanDecode<Via<Any, WasmClientState>>
{
}

impl CanUseWasmClientEncoding for WasmClientEncoding {}

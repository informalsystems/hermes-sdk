use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoderComponent, ClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::impls::decoders::client_state::DecodeWasmClientStateFromAnyProto;
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
            TimestampTypeComponent,
            ChainIdTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        ClientStateTypeComponent:
            ProvideWasmClientState,
        ClientStateDecoderComponent:
            DecodeWasmClientStateFromAnyProto,
    }
}

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
use hermes_wasm_client_components::impls::decoders::client_state::DecodeSovereignClientStateFromAnyProto;

use crate::sovereign::impls::types::client_state::ProvideSovereignClientState;

pub struct SovereignCounterparty;

pub struct SovereignCounterpartyComponents;

impl HasComponents for SovereignCounterparty {
    type Components = SovereignCounterpartyComponents;
}

delegate_components! {
    SovereignCounterpartyComponents {
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
            ProvideSovereignClientState,
        ClientStateDecoderComponent:
            DecodeSovereignClientStateFromAnyProto,
    }
}

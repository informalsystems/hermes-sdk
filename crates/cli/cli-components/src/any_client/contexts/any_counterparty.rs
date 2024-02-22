use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateDecoderComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateFieldsGetterComponent;
use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::any_client::impls::decoders::client_state::DecodeAnyClientState;
use crate::any_client::impls::types::client_state::ProvideAnyClientState;

pub struct AnyCounterparty;

pub struct AnyCounterpartyComponents;

impl HasComponents for AnyCounterparty {
    type Components = AnyCounterpartyComponents;
}

delegate_components! {
    AnyCounterpartyComponents {
        [
            HeightTypeComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
        ]:
            ProvideAnyClientState,
        ClientStateDecoderComponent:
            DecodeAnyClientState,
    }
}

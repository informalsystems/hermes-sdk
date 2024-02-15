use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStatesQuerierComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateTypeComponent, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use super::client_state::AnyClientState;
use super::counterparty::AnyCounterparty;
use super::queries::client_state::QueryAnyClientStatesFromChainHandle;

pub struct AnyCounterpartyComponents;

impl HasComponents for AnyCounterparty {
    type Components = AnyCounterpartyComponents;
}

delegate_components! {
    AnyCounterparty {
        [
            HeightTypeComponent,
            ChainIdTypeComponent,
            ClientStateTypeComponent,
        ]:
            AnyCounterpartyComponents,

        ClientStatesQuerierComponent:
            QueryAnyClientStatesFromChainHandle
    }
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
    }
}

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for AnyCounterpartyComponents
where
    Chain: Async,
{
    type ClientState = AnyClientState;
}

use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoderComponent, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::any::queries::client_state::DecodeAnyClientState;

use super::client_state::AnyClientState;
use super::counterparty::AnyCounterparty;

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
        ClientStateDecoderComponent:
            DecodeAnyClientState,
    }
}

impl<Chain, Counterparty> ProvideClientStateType<Chain, Counterparty> for AnyCounterpartyComponents
where
    Chain: Async,
{
    type ClientState = AnyClientState;
}

use core::time::Duration;

use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateDecoderComponent, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;

use crate::any_client::decoders::client_state::DecodeAnyClientState;
use crate::any_client::types::client_state::AnyClientState;

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

impl<Chain> HasClientStateFields<Chain> for AnyCounterparty {
    fn client_state_chain_id(client_state: &Self::ClientState) -> &Self::ChainId {
        match client_state {
            AnyClientState::Tendermint(cs) => &cs.chain_id,
        }
    }

    fn client_state_latest_height(client_state: &Self::ClientState) -> &Self::Height {
        match client_state {
            AnyClientState::Tendermint(cs) => &cs.latest_height,
        }
    }

    fn client_state_is_frozen(client_state: &Self::ClientState) -> bool {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.frozen_height.is_some(),
        }
    }

    fn client_state_has_expired(client_state: &Self::ClientState, elapsed: Duration) -> bool {
        match client_state {
            AnyClientState::Tendermint(cs) => cs.expired(elapsed),
        }
    }
}

use hermes_cli_components::any_client::contexts::any_counterparty::AnyCounterparty;
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryAllClientStates, CanQueryClientState,
};
use hermes_relayer_components_extra::components::extra::closures::chain::all::CanUseExtraChainComponents;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;

use crate::contexts::chain::CosmosChain;

impl CanUseExtraChainComponents<CosmosChain> for CosmosChain {}

pub trait CheckCosmosChainImpls:
    CanQueryBalance
    + CanIbcTransferToken<CosmosChain>
    + CanBuildIbcTokenTransferMessage<CosmosChain>
    + CanQueryClientState<AnyCounterparty>
    + CanQueryAllClientStates<AnyCounterparty>
    + CanAssertEventualAmount
{
}

impl CheckCosmosChainImpls for CosmosChain {}

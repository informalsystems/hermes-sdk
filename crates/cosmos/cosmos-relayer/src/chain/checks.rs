use hermes_cli_components::any_client::contexts::any_counterparty::AnyCounterparty;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_wasm_client_components::contexts::wasm_counterparty::WasmCounterparty;

use crate::contexts::chain::CosmosChain;

pub trait CheckCosmosChainImpls:
    CanQueryBalance
    + CanIbcTransferToken<CosmosChain>
    + CanBuildIbcTokenTransferMessage<CosmosChain>
    + CanQueryClientState<WasmCounterparty>
    + CanQueryClientState<AnyCounterparty>
    + CanAssertEventualAmount
{
}

impl CheckCosmosChainImpls for CosmosChain {}

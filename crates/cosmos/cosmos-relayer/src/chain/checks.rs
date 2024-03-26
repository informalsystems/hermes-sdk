use hermes_cli_components::any_client::contexts::any_counterparty::AnyCounterparty;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryAllClientStates, CanQueryClientState,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryConsensusState;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components_extra::components::extra::closures::chain::all::CanUseExtraChainComponents;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;

use crate::contexts::chain::CosmosChain;

impl CanUseExtraChainComponents<CosmosChain> for CosmosChain {}

pub trait CanUseCosmosChain:
    CanQueryBalance
    + CanIbcTransferToken<CosmosChain>
    + CanBuildIbcTokenTransferMessage<CosmosChain>
    + CanQueryClientState<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanQueryAllClientStates<CosmosChain>
    + CanQueryClientState<AnyCounterparty>
    + CanQueryAllClientStates<AnyCounterparty>
    + CanBuildUpdateClientMessage<CosmosChain>
    + CanSubmitTx
    + CanPollTxResponse
    + CanQueryTxResponse
    + CanAssertEventualAmount
{
}

impl CanUseCosmosChain for CosmosChain {}

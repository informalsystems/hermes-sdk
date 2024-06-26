use hermes_cli_components::any_client::contexts::any_counterparty::AnyCounterparty;
use hermes_cosmos_chain_components::types::commitment_proof::CosmosCommitmentProof;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    CanQueryChannelEnd, CanQueryChannelEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryAllClientStates, CanQueryClientState, CanQueryClientStateWithProofs,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs, CanQueryRawConsensusState,
};
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::CanQueryPacketAcknowledgement;
use hermes_relayer_components::chain::traits::queries::packet_commitment::CanQueryPacketCommitment;
use hermes_relayer_components::chain::traits::queries::packet_receipt::CanQueryPacketReceipt;
use hermes_relayer_components::chain::traits::types::channel::HasChannelEndType;
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateType, HasRawClientStateType,
};
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components_extra::components::extra::closures::chain::all::CanUseExtraChainComponents;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use ibc::core::channel::types::channel::ChannelEnd;
use prost_types::Any;

use crate::contexts::chain::CosmosChain;

impl CanUseExtraChainComponents<CosmosChain> for CosmosChain {}

pub trait CanUseCosmosChain:
    HasClientStateType<CosmosChain, ClientState = TendermintClientState>
    + HasChannelEndType<CosmosChain, ChannelEnd = ChannelEnd>
    + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
    + CanQueryBalance
    + CanIbcTransferToken<CosmosChain>
    + CanBuildIbcTokenTransferMessage<CosmosChain>
    + CanQueryClientState<CosmosChain>
    + CanQueryClientStateWithProofs<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanQueryConsensusStateWithProofs<CosmosChain>
    + CanQueryRawConsensusState<CosmosChain>
    + CanQueryAllClientStates<CosmosChain>
    + CanQueryClientState<AnyCounterparty>
    + CanQueryAllClientStates<AnyCounterparty>
    + CanBuildUpdateClientMessage<CosmosChain>
    + CanQueryConnectionEnd<CosmosChain>
    + CanQueryChannelEnd<CosmosChain>
    + CanQueryChannelEndWithProofs<CosmosChain>
    + CanQueryConnectionEndWithProofs<CosmosChain>
    + CanQueryPacketCommitment<CosmosChain>
    + CanQueryPacketAcknowledgement<CosmosChain>
    + CanQueryPacketReceipt<CosmosChain>
    + HasRawClientStateType<RawClientState = Any>
    + CanSubmitTx
    + CanPollTxResponse
    + CanQueryTxResponse
    + CanAssertEventualAmount
{
}

impl CanUseCosmosChain for CosmosChain {}

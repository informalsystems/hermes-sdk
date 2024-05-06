use cgp_core::prelude::*;
use hermes_test_components::chain::traits::proposal::types::proposal_id::ProposalIdTypeComponent;
use hermes_test_components::chain::traits::proposal::types::proposal_status::ProposalStatusTypeComponent;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::proposal::deposit::ProposalDepositerComponent;
use hermes_test_components::chain_driver::traits::proposal::poll_status::ProposalStatusPollerComponent;
use hermes_test_components::chain_driver::traits::proposal::query_status::ProposalStatusQuerierComponent;

use crate::chain_driver::impls::amount::GenerateRandomAmount;
use crate::chain_driver::impls::proposal::deposit::DepositProposalWithChainCommand;
use crate::chain_driver::impls::proposal::poll_status::PollProposalStatus;
use crate::chain_driver::impls::proposal::query_status::{
    ProvideCosmosProposalStatusType, QueryProposalStatusWithChainCommand,
};
use crate::chain_driver::impls::proposal::vote::VoteGovernanceProposalWithChainCommand;
use crate::chain_driver::impls::proposal_id::ProvideU64ProposalId;
use crate::chain_driver::traits::vote_proposal::GovernanceProposalVoterComponent;

pub struct CosmosChainDriverComponents;

delegate_components! {
    CosmosChainDriverComponents {
        RandomAmountGeneratorComponent:
            GenerateRandomAmount,
        ProposalIdTypeComponent:
            ProvideU64ProposalId,
        ProposalStatusTypeComponent:
            ProvideCosmosProposalStatusType,
        ProposalDepositerComponent:
            DepositProposalWithChainCommand,
        ProposalStatusQuerierComponent:
            QueryProposalStatusWithChainCommand,
        ProposalStatusPollerComponent:
            PollProposalStatus,
        GovernanceProposalVoterComponent:
            VoteGovernanceProposalWithChainCommand
    }
}

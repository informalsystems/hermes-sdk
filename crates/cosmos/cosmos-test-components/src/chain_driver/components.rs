use cgp_core::prelude::*;
use hermes_test_components::chain::traits::proposal::types::proposal_id::ProposalIdTypeComponent;
use hermes_test_components::chain::traits::proposal::types::proposal_status::ProposalStatusTypeComponent;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::proposal::deposit::ProposalDepositerComponent;

use crate::chain_driver::impls::amount::GenerateRandomAmount;
use crate::chain_driver::impls::deposit_proposal::DepositProposalWithChainCommand;
use crate::chain_driver::impls::poll_proposal_status::PollProposalStatus;
use crate::chain_driver::impls::proposal_id::ProvideU64ProposalId;
use crate::chain_driver::impls::proposal_status::{
    ProvideCosmosProposalStatusType, QueryGovernanceProposalStatusWithChainCommand,
};
use crate::chain_driver::impls::vote_proposal::VoteGovernanceProposalWithChainCommand;
use crate::chain_driver::traits::proposal_status::{
    GovernanceProposalStatusPollerComponent, GovernanceProposalStatusQuerierComponent,
};
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
        GovernanceProposalStatusQuerierComponent:
            QueryGovernanceProposalStatusWithChainCommand,
        GovernanceProposalStatusPollerComponent:
            PollProposalStatus,
        GovernanceProposalVoterComponent:
            VoteGovernanceProposalWithChainCommand
    }
}

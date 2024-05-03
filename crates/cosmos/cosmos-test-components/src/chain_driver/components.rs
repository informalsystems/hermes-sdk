use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;

use crate::chain_driver::impls::amount::GenerateRandomAmount;
use crate::chain_driver::impls::deposit_proposal::DepositGovernanceProposalWithChainCommand;
use crate::chain_driver::impls::proposal_status::QueryGovernanceProposalStatusWithChainCommand;
use crate::chain_driver::impls::vote_proposal::VoteGovernanceProposalWithChainCommand;
use crate::chain_driver::traits::deposit_proposal::GovernanceProposalDepositerComponent;
use crate::chain_driver::traits::proposal_status::GovernanceProposalStatusQuerierComponent;
use crate::chain_driver::traits::vote_proposal::GovernanceProposalVoterComponent;

pub struct CosmosChainDriverComponents;

delegate_components! {
    CosmosChainDriverComponents {
        RandomAmountGeneratorComponent:
            GenerateRandomAmount,
        GovernanceProposalDepositerComponent:
            DepositGovernanceProposalWithChainCommand,
        GovernanceProposalStatusQuerierComponent:
            QueryGovernanceProposalStatusWithChainCommand,
        GovernanceProposalVoterComponent:
            VoteGovernanceProposalWithChainCommand
    }
}

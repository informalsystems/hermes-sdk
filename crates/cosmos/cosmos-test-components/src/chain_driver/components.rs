use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::proposal::deposit::ProposalDepositerComponent;
use hermes_test_components::chain_driver::traits::proposal::poll_status::ProposalStatusPollerComponent;
use hermes_test_components::chain_driver::traits::proposal::query_status::ProposalStatusQuerierComponent;
use hermes_test_components::chain_driver::traits::proposal::vote::ProposalVoterComponent;

use crate::chain_driver::impls::amount::GenerateRandomAmount;
use crate::chain_driver::impls::proposal::deposit::DepositProposalWithChainCommand;
use crate::chain_driver::impls::proposal::poll_status::PollProposalStatus;
use crate::chain_driver::impls::proposal::query_status::QueryProposalStatusWithChainCommand;
use crate::chain_driver::impls::proposal::vote::VoteProposalWithChainCommand;

pub struct CosmosChainDriverComponents;

delegate_components! {
    CosmosChainDriverComponents {
        RandomAmountGeneratorComponent:
            GenerateRandomAmount,
        ProposalDepositerComponent:
            DepositProposalWithChainCommand,
        ProposalStatusQuerierComponent:
            QueryProposalStatusWithChainCommand,
        ProposalStatusPollerComponent:
            PollProposalStatus,
        ProposalVoterComponent:
            VoteProposalWithChainCommand
    }
}

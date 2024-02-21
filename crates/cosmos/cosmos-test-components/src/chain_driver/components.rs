use cgp_core::prelude::*;
use hermes_test_components::chain_driver::impls::default_assert_duration::ProvideDefaultPollAssertDuration;
use hermes_test_components::chain_driver::impls::poll_assert_eventual_amount::PollAssertEventualAmount;
use hermes_test_components::chain_driver::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain_driver::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;

use crate::chain_driver::impls::amount::GenerateRandomAmount;
use crate::chain_driver::impls::deposit_proposal::DepositGovernanceProposalWithChainCommand;
use crate::chain_driver::impls::proposal_status::QueryGovernanceProposalStatusWithChainCommand;
use crate::chain_driver::impls::store_wasm_client::UploadWasmClientCodeWithChainCommand;
use crate::chain_driver::impls::vote_proposal::VoteGovernanceProposalWithChainCommand;
use crate::chain_driver::traits::deposit_proposal::GovernanceProposalDepositerComponent;
use crate::chain_driver::traits::proposal_status::GovernanceProposalStatusQuerierComponent;
use crate::chain_driver::traits::store_wasm_client::WasmClientCodeUploaderComponent;
use crate::chain_driver::traits::vote_proposal::GovernanceProposalVoterComponent;

pub struct CosmosChainDriverComponents;

delegate_components! {
    CosmosChainDriverComponents {
        RandomAmountGeneratorComponent:
            GenerateRandomAmount,
        EventualAmountAsserterComponent:
            PollAssertEventualAmount,
        PollAssertDurationGetterComponent:
            ProvideDefaultPollAssertDuration,
        WasmClientCodeUploaderComponent:
            UploadWasmClientCodeWithChainCommand,
        GovernanceProposalDepositerComponent:
            DepositGovernanceProposalWithChainCommand,
        GovernanceProposalStatusQuerierComponent:
            QueryGovernanceProposalStatusWithChainCommand,
        GovernanceProposalVoterComponent:
            VoteGovernanceProposalWithChainCommand
    }
}

use core::fmt::{Debug, Display};
use core::time::Duration;

use cgp_core::error::CanRaiseError;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::HasProposalStatusType;
use hermes_test_components::chain_driver::traits::proposal::poll_status::ProposalStatusPoller;
use hermes_test_components::chain_driver::traits::proposal::query_status::CanQueryProposalStatus;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

pub struct PollProposalStatus;

impl<ChainDriver, Chain> ProposalStatusPoller<ChainDriver> for PollProposalStatus
where
    ChainDriver:
        HasChainType<Chain = Chain> + CanQueryProposalStatus + HasRuntime + CanRaiseError<String>,
    ChainDriver::Runtime: CanSleep,
    Chain::ProposalId: Display,
    Chain::ProposalStatus: Eq + Debug,
    Chain: HasProposalIdType + HasProposalStatusType,
{
    async fn poll_proposal_status(
        chain_driver: &ChainDriver,
        proposal_id: &Chain::ProposalId,
        expected_status: &Chain::ProposalStatus,
    ) -> Result<(), ChainDriver::Error> {
        let runtime = chain_driver.runtime();

        for _ in 0..20 {
            let status_result = chain_driver.query_proposal_status(proposal_id).await;
            match &status_result {
                Ok(status) if status == expected_status => {
                    return Ok(());
                }
                _ => {
                    runtime.sleep(Duration::from_millis(500)).await;
                }
            }
        }

        Err(ChainDriver::raise_error(format!(
            "Governance proposal {} was not in status {:?}",
            proposal_id, expected_status
        )))
    }
}

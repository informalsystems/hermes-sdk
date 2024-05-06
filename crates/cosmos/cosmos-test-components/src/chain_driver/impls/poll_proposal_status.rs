use core::fmt::{Debug, Display};
use core::time::Duration;

use cgp_core::CanRaiseError;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::chain_driver::traits::proposal_status::{
    CanQueryGovernanceProposalStatus, GovernanceProposalStatusPoller,
};

pub struct PollProposalStatus;

impl<ChainDriver> GovernanceProposalStatusPoller<ChainDriver> for PollProposalStatus
where
    ChainDriver: CanQueryGovernanceProposalStatus + HasRuntime + CanRaiseError<String>,
    ChainDriver::Runtime: CanSleep,
    ChainDriver::ProposalId: Display,
    ChainDriver::ProposalStatus: Eq + Debug,
{
    async fn poll_proposal_status(
        chain_driver: &ChainDriver,
        proposal_id: &ChainDriver::ProposalId,
        expected_status: &ChainDriver::ProposalStatus,
    ) -> Result<(), ChainDriver::Error> {
        let runtime = chain_driver.runtime();

        for _ in 0..20 {
            let status = chain_driver.query_proposal_status(proposal_id).await?;
            if &status == expected_status {
                return Ok(());
            } else {
                runtime.sleep(Duration::from_millis(500)).await;
            }
        }

        Err(ChainDriver::raise_error(format!(
            "Governance proposal {} was not in status {:?}",
            proposal_id, expected_status
        )))
    }
}

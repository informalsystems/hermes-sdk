use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum ProposalStatus {
    #[serde(rename = "PROPOSAL_STATUS_DEPOSIT_PERIOD")]
    DepositPeriod,

    #[serde(rename = "PROPOSAL_STATUS_VOTING_PERIOD")]
    VotingPeriod,

    #[serde(rename = "PROPOSAL_STATUS_PASSED")]
    Passed,

    #[serde(rename = "PROPOSAL_STATUS_REJECTED")]
    Rejected,

    #[serde(rename = "PROPOSAL_STATUS_FAILED")]
    Failed,
}

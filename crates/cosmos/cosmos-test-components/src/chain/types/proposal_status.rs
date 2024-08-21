#[derive(Debug, PartialEq, Eq)]
pub enum ProposalStatus {
    DepositPeriod,
    VotingPeriod,
    Passed,
    Rejected,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalStatus {
    DepositPeriod,
    VotingPeriod,
    Passed,
    Rejected,
    Failed,
}

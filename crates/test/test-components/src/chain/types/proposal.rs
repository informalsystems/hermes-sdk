#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalStatus {
    DepositPeriod,
    VotingPeriod,
    Passed,
    Rejected,
}

#[derive(Debug)]
pub enum ProposalVote {
    Yes,
    Abstain,
    No,
    NoWithVeto,
}

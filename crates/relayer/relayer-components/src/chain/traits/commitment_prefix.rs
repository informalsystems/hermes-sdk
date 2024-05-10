use cgp_core::prelude::*;

#[derive_component(CommitmentPrefixTypeComponent, ProvideCommitmentPrefixType<Chain>)]
pub trait HasCommitmentPrefixType: Async {
    type CommitmentPrefix: Async;
}

#[derive_component(IbcCommitmentPrefixGetterComponent, IbcCommitmentPrefixGetter<Chain>)]
pub trait HasIbcCommitmentPrefix: HasCommitmentPrefixType {
    fn ibc_commitment_prefix(&self) -> &Self::CommitmentPrefix;
}

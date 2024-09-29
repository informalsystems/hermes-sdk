use cgp::prelude::*;

pub use hermes_chain_type_components::traits::types::commitment_prefix::*;

#[derive_component(IbcCommitmentPrefixGetterComponent, IbcCommitmentPrefixGetter<Chain>)]
pub trait HasIbcCommitmentPrefix: HasCommitmentPrefixType {
    fn ibc_commitment_prefix(&self) -> &Self::CommitmentPrefix;
}

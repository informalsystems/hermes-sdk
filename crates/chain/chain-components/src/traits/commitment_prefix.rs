use cgp::prelude::*;
pub use hermes_chain_type_components::traits::types::commitment_prefix::*;

#[cgp_component {
  name: IbcCommitmentPrefixGetterComponent,
  provider: IbcCommitmentPrefixGetter,
  context: Chain,
}]
pub trait HasIbcCommitmentPrefix: HasCommitmentPrefixType {
    fn ibc_commitment_prefix(&self) -> &Self::CommitmentPrefix;
}

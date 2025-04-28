pub use hermes_chain_type_components::traits::*;
use hermes_prelude::*;

#[cgp_component {
  provider: IbcCommitmentPrefixGetter,
  context: Chain,
}]
pub trait HasIbcCommitmentPrefix: HasCommitmentPrefixType {
    fn ibc_commitment_prefix(&self) -> &Self::CommitmentPrefix;
}

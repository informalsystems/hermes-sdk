use alloc::vec::Vec;

use hermes_chain_type_components::traits::CommitmentPrefixTypeComponent;
use hermes_prelude::*;

use crate::traits::ProvideCommitmentPrefixType;

pub struct ProvideCommitmentPrefixBytes;

#[cgp_provider(CommitmentPrefixTypeComponent)]
impl<Chain> ProvideCommitmentPrefixType<Chain> for ProvideCommitmentPrefixBytes
where
    Chain: Async,
{
    type CommitmentPrefix = Vec<u8>;
}

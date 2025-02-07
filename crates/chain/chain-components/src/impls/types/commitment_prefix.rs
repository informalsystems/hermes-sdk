use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::commitment_prefix::CommitmentPrefixTypeComponent;

use crate::traits::commitment_prefix::ProvideCommitmentPrefixType;

pub struct ProvideCommitmentPrefixBytes;

#[cgp_provider(CommitmentPrefixTypeComponent)]
impl<Chain> ProvideCommitmentPrefixType<Chain> for ProvideCommitmentPrefixBytes
where
    Chain: Async,
{
    type CommitmentPrefix = Vec<u8>;
}

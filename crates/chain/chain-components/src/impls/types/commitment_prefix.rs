use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::CommitmentPrefixTypeComponent;

use crate::traits::ProvideCommitmentPrefixType;

pub struct ProvideCommitmentPrefixBytes;

#[cgp_provider(CommitmentPrefixTypeComponent)]
impl<Chain> ProvideCommitmentPrefixType<Chain> for ProvideCommitmentPrefixBytes
where
    Chain: Async,
{
    type CommitmentPrefix = Vec<u8>;
}

use alloc::vec::Vec;
use std::sync::OnceLock;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::commitment_prefix::{
    HasCommitmentPrefixType, IbcCommitmentPrefixGetter, IbcCommitmentPrefixGetterComponent,
};

pub struct ProvideIbcCommitmentPrefix;

#[cgp_provider(IbcCommitmentPrefixGetterComponent)]
impl<Chain> IbcCommitmentPrefixGetter<Chain> for ProvideIbcCommitmentPrefix
where
    Chain: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>,
{
    fn ibc_commitment_prefix(_chain: &Chain) -> &Vec<u8> {
        static IBC_COMMITMENT_PREFIX: OnceLock<Vec<u8>> = OnceLock::new();

        IBC_COMMITMENT_PREFIX.get_or_init(|| "ibc".into())
    }
}

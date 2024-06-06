use alloc::vec::Vec;
use std::sync::OnceLock;

use hermes_relayer_components::chain::traits::commitment_prefix::{
    HasCommitmentPrefixType, IbcCommitmentPrefixGetter,
};

pub struct ProvideSovereignIbcCommitmentPrefix;

impl<Chain> IbcCommitmentPrefixGetter<Chain> for ProvideSovereignIbcCommitmentPrefix
where
    Chain: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>,
{
    fn ibc_commitment_prefix(_chain: &Chain) -> &Vec<u8> {
        static IBC_COMMITMENT_PREFIX: OnceLock<Vec<u8>> = OnceLock::new();

        IBC_COMMITMENT_PREFIX.get_or_init(|| "sov_ibc/Ibc/".into())
    }
}

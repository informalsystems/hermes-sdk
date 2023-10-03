use cgp_core::HasErrorType;

use crate::one_for_all::traits::chain::OfaChain;
use crate::one_for_all::types::chain::OfaChainWrapper;

impl<Chain: OfaChain> HasErrorType for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    type Error = Chain::Error;
}

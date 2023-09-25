use cgp_core::traits::error::HasErrorType;

use crate::one_for_all::traits::birelay::OfaBiRelay;
use crate::one_for_all::types::birelay::OfaBiRelayWrapper;

impl<BiRelay> HasErrorType for OfaBiRelayWrapper<BiRelay>
where
    BiRelay: OfaBiRelay,
{
    type Error = BiRelay::Error;
}

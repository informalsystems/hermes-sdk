use cgp::core::error::ErrorOf;
use cgp::core::macros::blanket_trait;
use hermes_prelude::*;

use crate::relay::traits::HasRelayChainTypes;

#[blanket_trait]
pub trait CanRaiseRelayChainErrors:
    HasRelayChainTypes
    + CanRaiseAsyncError<ErrorOf<Self::SrcChain>>
    + CanRaiseAsyncError<ErrorOf<Self::DstChain>>
{
}

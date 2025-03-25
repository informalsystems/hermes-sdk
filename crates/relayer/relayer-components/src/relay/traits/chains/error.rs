use cgp::core::error::ErrorOf;
use cgp::core::macros::blanket_trait;
use cgp::prelude::*;

use crate::relay::traits::chains::types::HasRelayChainTypes;

#[blanket_trait]
pub trait CanRaiseRelayChainErrors:
    HasRelayChainTypes
    + CanRaiseAsyncError<ErrorOf<Self::SrcChain>>
    + CanRaiseAsyncError<ErrorOf<Self::DstChain>>
{
}

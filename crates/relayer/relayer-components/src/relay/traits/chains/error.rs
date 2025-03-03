use cgp::core::error::ErrorOf;
use cgp::core::macros::trait_alias;
use cgp::prelude::*;

use crate::relay::traits::chains::types::HasRelayChainTypes;

#[trait_alias]
pub trait CanRaiseRelayChainErrors:
    HasRelayChainTypes
    + CanRaiseAsyncError<ErrorOf<Self::SrcChain>>
    + CanRaiseAsyncError<ErrorOf<Self::DstChain>>
{
}

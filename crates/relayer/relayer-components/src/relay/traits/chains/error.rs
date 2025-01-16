use cgp::core::error::ErrorOf;
use cgp::prelude::*;

use crate::relay::traits::chains::types::HasRelayChainTypes;

pub trait CanRaiseRelayChainErrors:
    HasRelayChainTypes
    + CanRaiseAsyncError<ErrorOf<Self::SrcChain>>
    + CanRaiseAsyncError<ErrorOf<Self::DstChain>>
{
}

impl<Relay> CanRaiseRelayChainErrors for Relay where
    Relay: HasRelayChainTypes
        + CanRaiseAsyncError<ErrorOf<Self::SrcChain>>
        + CanRaiseAsyncError<ErrorOf<Self::DstChain>>
{
}

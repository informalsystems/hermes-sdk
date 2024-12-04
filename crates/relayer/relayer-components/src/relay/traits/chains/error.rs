use cgp::core::error::ErrorOf;
use cgp::prelude::*;

use crate::relay::traits::chains::types::HasRelayChainTypes;

pub trait CanRaiseRelayChainErrors:
    HasRelayChainTypes + CanRaiseError<ErrorOf<Self::SrcChain>> + CanRaiseError<ErrorOf<Self::DstChain>>
{
}

impl<Relay> CanRaiseRelayChainErrors for Relay where
    Relay: HasRelayChainTypes
        + CanRaiseError<ErrorOf<Self::SrcChain>>
        + CanRaiseError<ErrorOf<Self::DstChain>>
{
}

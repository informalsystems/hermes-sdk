use core::marker::PhantomData;

use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::client_id_at::HasClientIdAt;
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::{HasRelayChainTypes, HasRelayChains};

pub trait HasSrcClientId: HasRelayChainTypes {
    fn src_client_id(&self) -> &ClientIdOf<Self::SrcChain, Self::DstChain>;
}

pub trait HasDstClientId: HasRelayChainTypes {
    fn dst_client_id(&self) -> &ClientIdOf<Self::DstChain, Self::SrcChain>;
}

pub trait HasRelayClientIds: HasRelayChains + HasSrcClientId + HasDstClientId {}

impl<Relay> HasRelayClientIds for Relay where Relay: HasRelayChains + HasSrcClientId + HasDstClientId
{}

impl<Relay> HasSrcClientId for Relay
where
    Relay: HasRelayChainTypes + HasClientIdAt<Src, Dst>,
{
    fn src_client_id(&self) -> &ClientIdOf<Relay::SrcChain, Relay::DstChain> {
        self.client_id_at(PhantomData)
    }
}

impl<Relay> HasDstClientId for Relay
where
    Relay: HasRelayChainTypes + HasClientIdAt<Dst, Src>,
{
    fn dst_client_id(&self) -> &ClientIdOf<Relay::DstChain, Relay::SrcChain> {
        self.client_id_at(PhantomData)
    }
}

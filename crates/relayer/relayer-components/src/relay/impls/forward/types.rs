use cgp_core::prelude::HasErrorType;
use cgp_core::HasInner;

use crate::chain::types::aliases::ClientIdOf;
use crate::relay::traits::chains::{HasRelayChains, ProvideRelayChains};

pub struct ForwardRelayTypes;

impl<Relay, Inner> ProvideRelayChains<Relay> for ForwardRelayTypes
where
    Relay: HasInner<Inner = Inner> + HasErrorType,
    Inner: HasRelayChains,
{
    type Packet = Inner::Packet;

    type SrcChain = Inner::SrcChain;

    type DstChain = Inner::DstChain;

    fn src_chain(relay: &Relay) -> &Self::SrcChain {
        relay.inner().src_chain()
    }

    fn dst_chain(relay: &Relay) -> &Self::DstChain {
        relay.inner().dst_chain()
    }

    fn src_client_id(relay: &Relay) -> &ClientIdOf<Self::SrcChain, Self::DstChain> {
        relay.inner().src_client_id()
    }

    fn dst_client_id(relay: &Relay) -> &ClientIdOf<Self::DstChain, Self::SrcChain> {
        relay.inner().dst_client_id()
    }
}

use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::types::aliases::ClientIdOf;

use crate::relay::traits::chains::{HasRelayChains, RelayClientIdGetter};

pub struct UseClientIdFields<SrcClientId, DstClientId>(pub PhantomData<(SrcClientId, DstClientId)>);

pub type UseDefaultClientIdFields =
    UseClientIdFields<symbol!("src_client_id"), symbol!("dst_client_id")>;

impl<Relay, SrcClientId, DstClientId> RelayClientIdGetter<Relay>
    for UseClientIdFields<SrcClientId, DstClientId>
where
    Relay: HasRelayChains
        + HasField<SrcClientId, Field = ClientIdOf<Relay::SrcChain, Relay::DstChain>>
        + HasField<DstClientId, Field = ClientIdOf<Relay::DstChain, Relay::SrcChain>>,
{
    fn src_client_id(relay: &Relay) -> &ClientIdOf<Relay::SrcChain, Relay::DstChain> {
        relay.get_field(PhantomData::<SrcClientId>)
    }

    fn dst_client_id(relay: &Relay) -> &ClientIdOf<Relay::DstChain, Relay::SrcChain> {
        relay.get_field(PhantomData::<DstClientId>)
    }
}

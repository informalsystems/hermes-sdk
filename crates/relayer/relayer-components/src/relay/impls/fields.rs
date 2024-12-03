use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::types::aliases::ClientIdOf;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::relay::traits::chains::{
    HasRelayChainTypes, HasRelayChains, ProvideRelayChainTypes, ProvideRelayChains,
    RelayClientIdGetter,
};

pub struct UseRelayFields<SrcChainField, DstChainField>(
    pub PhantomData<(SrcChainField, DstChainField)>,
);

pub struct UseClientIdFields<SrcClientId, DstClientId>(pub PhantomData<(SrcClientId, DstClientId)>);

pub type UseDefaultRelayFields = UseRelayFields<symbol!("src_chain"), symbol!("dst_chain")>;

pub type UseDefaultClientIdFields =
    UseClientIdFields<symbol!("src_client_id"), symbol!("dst_client_id")>;

impl<Relay, SrcChain, DstChain, SrcChainField: Async, DstChainField: Async>
    ProvideRelayChainTypes<Relay> for UseRelayFields<SrcChainField, DstChainField>
where
    Relay: Async
        + HasErrorType
        + HasField<SrcChainField, Field = SrcChain>
        + HasField<DstChainField, Field = DstChain>,
    SrcChain: HasErrorType + HasIbcChainTypes<DstChain> + HasOutgoingPacketType<DstChain>,
    DstChain: HasErrorType + HasIbcChainTypes<SrcChain>,
{
    type SrcChain = SrcChain;

    type DstChain = DstChain;
}

impl<Relay, SrcChainField: Async, DstChainField: Async> ProvideRelayChains<Relay>
    for UseRelayFields<SrcChainField, DstChainField>
where
    Relay: HasRelayChainTypes
        + HasField<SrcChainField, Field = Relay::SrcChain>
        + HasField<DstChainField, Field = Relay::DstChain>,
{
    fn src_chain(relay: &Relay) -> &Relay::SrcChain {
        relay.get_field(PhantomData::<SrcChainField>)
    }

    fn dst_chain(relay: &Relay) -> &Relay::DstChain {
        relay.get_field(PhantomData::<DstChainField>)
    }
}

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

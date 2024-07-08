use core::marker::PhantomData;

use cgp_core::error::HasErrorType;
use cgp_core::field::HasField;
use cgp_core::Async;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::packet::HasIbcPacketTypes;
use crate::relay::traits::chains::ProvideRelayChains;

pub struct ProvideRelayFields<SrcChainField, DstChainField, SrcClientIdField, DstClientIdField>(
    pub  PhantomData<(
        SrcChainField,
        DstChainField,
        SrcClientIdField,
        DstClientIdField,
    )>,
);

impl<
        Relay,
        SrcChain,
        DstChain,
        Packet: Async,
        SrcChainField: Async,
        DstChainField: Async,
        SrcClientIdField: Async,
        DstClientIdField: Async,
    > ProvideRelayChains<Relay>
    for ProvideRelayFields<SrcChainField, DstChainField, SrcClientIdField, DstClientIdField>
where
    Relay: HasErrorType
        + HasField<SrcChainField, Field = SrcChain>
        + HasField<DstChainField, Field = DstChain>
        + HasField<SrcClientIdField, Field = SrcChain::ClientId>
        + HasField<DstClientIdField, Field = DstChain::ClientId>,
    SrcChain: HasErrorType
        + HasIbcChainTypes<DstChain>
        + HasIbcPacketTypes<DstChain, OutgoingPacket = Packet>,
    DstChain: HasErrorType
        + HasIbcChainTypes<SrcChain>
        + HasIbcPacketTypes<SrcChain, IncomingPacket = Packet>,
{
    type SrcChain = SrcChain;

    type DstChain = DstChain;

    type Packet = Packet;

    fn src_chain(relay: &Relay) -> &SrcChain {
        &relay.get_field(PhantomData::<SrcChainField>)
    }

    fn dst_chain(relay: &Relay) -> &DstChain {
        &relay.get_field(PhantomData::<DstChainField>)
    }

    fn src_client_id(relay: &Relay) -> &SrcChain::ClientId {
        &relay.get_field(PhantomData::<SrcClientIdField>)
    }

    fn dst_client_id(relay: &Relay) -> &DstChain::ClientId {
        &relay.get_field(PhantomData::<DstClientIdField>)
    }
}

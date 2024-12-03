use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{HasChainAt, HasChainTypeAt};
use crate::multi::types::tags::{Dst, Src};

pub trait HasRelayChainTypes: HasChainTypeAt<Src> + HasChainTypeAt<Dst> + HasErrorType {
    type SrcChain: HasErrorType
        + HasIbcChainTypes<Self::DstChain>
        + HasOutgoingPacketType<Self::DstChain>;

    type DstChain: HasErrorType + HasIbcChainTypes<Self::SrcChain>;
}

impl<Relay, SrcChain, DstChain> HasRelayChainTypes for Relay
where
    Relay: HasChainTypeAt<Src, Chain = SrcChain>
        + HasChainTypeAt<Dst, Chain = DstChain>
        + HasErrorType,
    SrcChain: HasErrorType + HasIbcChainTypes<DstChain> + HasOutgoingPacketType<DstChain>,
    DstChain: HasErrorType + HasIbcChainTypes<SrcChain>,
{
    type SrcChain = SrcChain;

    type DstChain = DstChain;
}

pub trait HasRelayChains: HasRelayChainTypes {
    fn src_chain(&self) -> &Self::SrcChain;

    fn dst_chain(&self) -> &Self::DstChain;
}

impl<Relay, SrcChain, DstChain> HasRelayChains for Relay
where
    Relay: HasChainAt<Src, Chain = SrcChain> + HasChainAt<Dst, Chain = DstChain> + HasErrorType,
    SrcChain: HasErrorType + HasIbcChainTypes<DstChain> + HasOutgoingPacketType<DstChain>,
    DstChain: HasErrorType + HasIbcChainTypes<SrcChain>,
{
    fn src_chain(&self) -> &Self::SrcChain {
        self.chain_at(PhantomData::<Src>)
    }

    fn dst_chain(&self) -> &Self::DstChain {
        self.chain_at(PhantomData::<Dst>)
    }
}

#[derive_component(SrcClientIdGetterComponent, SrcClientIdGetter<Relay>)]
pub trait HasSrcClientId: HasRelayChainTypes {
    fn src_client_id(&self) -> &ClientIdOf<Self::SrcChain, Self::DstChain>;
}

#[derive_component(DstClientIdGetterComponent, DstClientIdGetter<Relay>)]
pub trait HasDstClientId: HasRelayChainTypes {
    fn dst_client_id(&self) -> &ClientIdOf<Self::DstChain, Self::SrcChain>;
}

pub trait HasRelayClientIds: HasRelayChains + HasSrcClientId + HasDstClientId {}

impl<Relay> HasRelayClientIds for Relay where Relay: HasRelayChains + HasSrcClientId + HasDstClientId
{}

impl<Relay, Tag> SrcClientIdGetter<Relay> for UseField<Tag>
where
    Relay: HasRelayChainTypes + HasField<Tag, Field = ClientIdOf<Relay::SrcChain, Relay::DstChain>>,
{
    fn src_client_id(relay: &Relay) -> &ClientIdOf<Relay::SrcChain, Relay::DstChain> {
        relay.get_field(PhantomData)
    }
}

impl<Relay, Tag> DstClientIdGetter<Relay> for UseField<Tag>
where
    Relay: HasRelayChainTypes + HasField<Tag, Field = ClientIdOf<Relay::DstChain, Relay::SrcChain>>,
{
    fn dst_client_id(relay: &Relay) -> &ClientIdOf<Relay::DstChain, Relay::SrcChain> {
        relay.get_field(PhantomData)
    }
}

pub type SrcChainOf<Relay> = <Relay as HasRelayChainTypes>::SrcChain;

pub type DstChainOf<Relay> = <Relay as HasRelayChainTypes>::DstChain;

pub type PacketOf<Relay> =
    <SrcChainOf<Relay> as HasOutgoingPacketType<DstChainOf<Relay>>>::OutgoingPacket;

pub trait CanRaiseRelayChainErrors:
    HasRelayChains + CanRaiseError<ErrorOf<Self::SrcChain>> + CanRaiseError<ErrorOf<Self::DstChain>>
{
}

impl<Relay> CanRaiseRelayChainErrors for Relay where
    Relay: HasRelayChains
        + CanRaiseError<ErrorOf<Self::SrcChain>>
        + CanRaiseError<ErrorOf<Self::DstChain>>
{
}

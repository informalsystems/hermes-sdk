use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::prelude::*;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{HasChainAt, HasChainTypeAt};
use crate::multi::traits::client_id_at::HasClientIdAt;
use crate::multi::types::tags::{Dst, Src};

pub trait HasSrcChainType: HasChainTypeAt<Src, Chain = Self::SrcChain> {
    type SrcChain;
}

pub trait HasDstChainType: HasChainTypeAt<Dst, Chain = Self::DstChain> {
    type DstChain;
}

pub trait HasSrcChain: HasSrcChainType {
    fn src_chain(&self) -> &Self::SrcChain;
}

pub trait HasDstChain: HasDstChainType {
    fn dst_chain(&self) -> &Self::DstChain;
}

impl<Relay> HasSrcChainType for Relay
where
    Relay: HasChainTypeAt<Src>,
{
    type SrcChain = Relay::Chain;
}

impl<Relay> HasDstChainType for Relay
where
    Relay: HasChainTypeAt<Dst>,
{
    type DstChain = Relay::Chain;
}

impl<Relay> HasSrcChain for Relay
where
    Relay: HasChainAt<Src>,
{
    fn src_chain(&self) -> &Self::SrcChain {
        self.chain_at(PhantomData)
    }
}

impl<Relay> HasDstChain for Relay
where
    Relay: HasChainAt<Dst>,
{
    fn dst_chain(&self) -> &Self::DstChain {
        self.chain_at(PhantomData)
    }
}

pub trait HasRelayChainTypes:
    HasErrorType
    + HasSrcChainType<
        SrcChain: HasErrorType
                      + HasIbcChainTypes<Self::DstChain>
                      + HasOutgoingPacketType<Self::DstChain>,
    > + HasDstChainType<DstChain: HasErrorType + HasIbcChainTypes<Self::SrcChain>>
{
}

impl<Relay, SrcChain, DstChain> HasRelayChainTypes for Relay
where
    Relay: HasChainTypeAt<Src, Chain = SrcChain>
        + HasChainTypeAt<Dst, Chain = DstChain>
        + HasErrorType,
    SrcChain: HasErrorType + HasIbcChainTypes<DstChain> + HasOutgoingPacketType<DstChain>,
    DstChain: HasErrorType + HasIbcChainTypes<SrcChain>,
{
}

pub trait HasRelayChains: HasRelayChainTypes + HasSrcChain + HasDstChain {}

impl<Relay> HasRelayChains for Relay where Relay: HasRelayChainTypes + HasSrcChain + HasDstChain {}

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

pub type SrcChainOf<Relay> = <Relay as HasSrcChainType>::SrcChain;

pub type DstChainOf<Relay> = <Relay as HasDstChainType>::DstChain;

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

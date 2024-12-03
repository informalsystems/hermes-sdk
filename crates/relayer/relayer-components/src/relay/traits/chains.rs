use cgp::core::error::ErrorOf;
use cgp::prelude::*;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;

#[derive_component(RelayChainsComponent, ProvideRelayChains<Relay>)]
pub trait HasRelayChains: Async + HasErrorType {
    type SrcChain: HasErrorType
        + HasIbcChainTypes<Self::DstChain>
        + HasOutgoingPacketType<Self::DstChain>;

    type DstChain: HasErrorType + HasIbcChainTypes<Self::SrcChain>;

    fn src_chain(&self) -> &Self::SrcChain;

    fn dst_chain(&self) -> &Self::DstChain;
}

#[derive_component(RelayClientIdGetterComponent, RelayClientIdGetter<Relay>)]
pub trait HasRelayClientIds: HasRelayChains {
    fn src_client_id(&self) -> &ClientIdOf<Self::SrcChain, Self::DstChain>;

    fn dst_client_id(&self) -> &ClientIdOf<Self::DstChain, Self::SrcChain>;
}

pub type SrcChainOf<Relay> = <Relay as HasRelayChains>::SrcChain;

pub type DstChainOf<Relay> = <Relay as HasRelayChains>::DstChain;

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

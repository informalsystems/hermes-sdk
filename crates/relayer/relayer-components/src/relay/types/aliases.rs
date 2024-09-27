use hermes_chain_components::types::aliases::{ChannelIdOf, ConnectionIdOf, PortIdOf};

use crate::relay::traits::chains::HasRelayChains;

pub type Packet<Relay> = <Relay as HasRelayChains>::Packet;

pub type SrcChain<Relay> = <Relay as HasRelayChains>::SrcChain;

pub type DstChain<Relay> = <Relay as HasRelayChains>::DstChain;

pub type SrcConnectionId<Relay> = ConnectionIdOf<SrcChain<Relay>, DstChain<Relay>>;

pub type DstConnectionId<Relay> = ConnectionIdOf<DstChain<Relay>, SrcChain<Relay>>;

pub type SrcPortId<Relay> = PortIdOf<SrcChain<Relay>, DstChain<Relay>>;

pub type DstPortId<Relay> = PortIdOf<DstChain<Relay>, SrcChain<Relay>>;

pub type SrcChannelId<Relay> = ChannelIdOf<SrcChain<Relay>, DstChain<Relay>>;

pub type DstChannelId<Relay> = ChannelIdOf<DstChain<Relay>, SrcChain<Relay>>;

use async_trait::async_trait;
use cgp_core::traits::delegate_component::DelegateComponent;
use cgp_core::traits::has_components::HasComponents;
use cgp_core::traits::Async;

use crate::relay::traits::chains::HasRelayChains;
use crate::std_prelude::*;

pub struct PacketFilterComponent;

#[async_trait]
pub trait PacketFilter<Relay>: Async
where
    Relay: HasRelayChains,
{
    async fn should_relay_packet(
        relay: &Relay,
        packet: &Relay::Packet,
    ) -> Result<bool, Relay::Error>;
}

#[async_trait]
impl<Relay, Component> PacketFilter<Relay> for Component
where
    Relay: HasRelayChains,
    Component: DelegateComponent<PacketFilterComponent>,
    Component::Delegate: PacketFilter<Relay>,
{
    async fn should_relay_packet(
        relay: &Relay,
        packet: &Relay::Packet,
    ) -> Result<bool, Relay::Error> {
        Component::Delegate::should_relay_packet(relay, packet).await
    }
}

#[async_trait]
pub trait CanFilterPackets: HasRelayChains {
    async fn should_relay_packet(&self, packet: &Self::Packet) -> Result<bool, Self::Error>;
}

#[async_trait]
impl<Relay> CanFilterPackets for Relay
where
    Relay: HasRelayChains + HasComponents,
    Relay::Components: PacketFilter<Relay>,
{
    async fn should_relay_packet(&self, packet: &Self::Packet) -> Result<bool, Self::Error> {
        Relay::Components::should_relay_packet(self, packet).await
    }
}

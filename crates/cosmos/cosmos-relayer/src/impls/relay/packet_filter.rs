use async_trait::async_trait;
use hermes_relayer_components::relay::traits::components::packet_filter::PacketFilter;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::contexts::relay::CosmosRelay;
use crate::impls::relay::component::CosmosRelayComponents;
use crate::types::error::Error;

#[async_trait]
impl<SrcChain, DstChain> PacketFilter<CosmosRelay<SrcChain, DstChain>> for CosmosRelayComponents
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
    async fn should_relay_packet(
        relay: &CosmosRelay<SrcChain, DstChain>,
        packet: &Packet,
    ) -> Result<bool, Error> {
        Ok(relay
            .packet_filter
            .channel_policy
            .is_allowed(&packet.source_port, &packet.source_channel))
    }
}

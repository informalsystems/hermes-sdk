use cgp::prelude::*;
use hermes_chain_components::traits::packet::fields::{
    HasPacketDstChannelId, HasPacketDstPortId, HasPacketSrcChannelId, HasPacketSrcPortId,
};

use crate::chain::traits::queries::counterparty_chain_id::CanQueryCounterpartyChainId;
use crate::chain::traits::types::chain_id::HasChainId;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, PacketOf};
use crate::relay::traits::packet_filter::{RelayPacketFilter, RelayPacketFilterComponent};

pub struct MatchPacketSourceChain;

pub struct MatchPacketDestinationChain;

#[cgp_provider(RelayPacketFilterComponent)]
impl<Relay> RelayPacketFilter<Relay> for MatchPacketSourceChain
where
    Relay: HasRelayChains + CanRaiseRelayChainErrors,
    Relay::DstChain: CanQueryCounterpartyChainId<Relay::SrcChain>,
    Relay::SrcChain:
        HasChainId + HasPacketDstChannelId<Relay::DstChain> + HasPacketDstPortId<Relay::DstChain>,
{
    async fn should_relay_packet(
        relay: &Relay,
        packet: &PacketOf<Relay>,
    ) -> Result<bool, Relay::Error> {
        let dst_channel_id = Relay::SrcChain::packet_dst_channel_id(packet);
        let dst_port = Relay::SrcChain::packet_dst_port_id(packet);

        let src_chain_id = relay
            .dst_chain()
            .query_counterparty_chain_id_from_channel_id(&dst_channel_id, &dst_port)
            .await
            .map_err(Relay::raise_error)?;

        let same_chain = &src_chain_id == relay.src_chain().chain_id();

        Ok(same_chain)
    }
}

#[cgp_provider(RelayPacketFilterComponent)]
impl<Relay> RelayPacketFilter<Relay> for MatchPacketDestinationChain
where
    Relay: HasRelayChains + CanRaiseRelayChainErrors,
    Relay::SrcChain: CanQueryCounterpartyChainId<Relay::DstChain>
        + HasPacketSrcChannelId<Relay::DstChain>
        + HasPacketSrcPortId<Relay::DstChain>,
    Relay::DstChain: HasChainId,
{
    async fn should_relay_packet(
        relay: &Relay,
        packet: &PacketOf<Relay>,
    ) -> Result<bool, Relay::Error> {
        let src_channel_id = Relay::SrcChain::packet_src_channel_id(packet);
        let src_port = Relay::SrcChain::packet_src_port_id(packet);

        let dst_chain_id = relay
            .src_chain()
            .query_counterparty_chain_id_from_channel_id(&src_channel_id, &src_port)
            .await
            .map_err(Relay::raise_error)?;

        let same_chain = &dst_chain_id == relay.dst_chain().chain_id();

        Ok(same_chain)
    }
}

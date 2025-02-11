use cgp::prelude::*;
use futures::stream::{self, StreamExt, TryStreamExt};
use hermes_relayer_components::chain::traits::queries::ack_packets::{
    AckPacketsQuerier, AckPacketsQuerierComponent, CanQueryAckPacket,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::types::aliases::WriteAckEventOf;

pub struct QueryAckPacketsConcurrently;

#[cgp_provider(AckPacketsQuerierComponent)]
impl<Chain, Counterparty> AckPacketsQuerier<Chain, Counterparty> for QueryAckPacketsConcurrently
where
    Chain: CanQueryAckPacket<Counterparty>,
    Counterparty: HasIbcChainTypes<Chain> + HasWriteAckEvent<Chain> + HasOutgoingPacketType<Chain>,
{
    async fn query_ack_packets_from_sequences(
        chain: &Chain,
        channel_id: &Counterparty::ChannelId,
        port_id: &Counterparty::PortId,
        counterparty_channel_id: &Chain::ChannelId,
        counterparty_port_id: &Chain::PortId,
        sequences: &[Counterparty::Sequence],
        height: &Chain::Height,
    ) -> Result<
        Vec<(
            Counterparty::OutgoingPacket,
            WriteAckEventOf<Chain, Counterparty>,
        )>,
        Chain::Error,
    > {
        let ack_packets = stream::iter(sequences)
            // TODO: use `flat_map_unordered`
            .then(|sequence| {
                chain.query_ack_packet_from_sequence(
                    channel_id,
                    port_id,
                    counterparty_channel_id,
                    counterparty_port_id,
                    sequence,
                    height,
                )
            })
            .try_collect::<Vec<_>>()
            .await?;

        Ok(ack_packets)
    }
}

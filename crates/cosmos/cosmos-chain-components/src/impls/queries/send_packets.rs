use cgp::prelude::*;
use futures::stream::{self, StreamExt, TryStreamExt};
use hermes_relayer_components::chain::traits::queries::send_packets::{
    CanQuerySendPacket, SendPacketsQuerier, SendPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

pub struct QuerySendPacketsConcurrently;

#[cgp_provider(SendPacketsQuerierComponent)]
impl<Chain, Counterparty> SendPacketsQuerier<Chain, Counterparty> for QuerySendPacketsConcurrently
where
    Chain: CanQuerySendPacket<Counterparty>,
    Counterparty: HasIbcChainTypes<Chain>,
{
    async fn query_send_packets_from_sequences(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_port_id: &Counterparty::PortId,
        sequences: &[Chain::Sequence],
        height: &Chain::Height,
    ) -> Result<Vec<Chain::OutgoingPacket>, Chain::Error> {
        let send_packets = stream::iter(sequences)
            // TODO: use `flat_map_unordered`
            .then(|sequence| {
                chain.query_send_packet_from_sequence(
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

        Ok(send_packets)
    }
}

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::components::received_packet_querier::ReceivedPacketQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::QueryUnreceivedPacketsRequest;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::traits::chain_handle::HasBlockingChainHandle;

pub struct QueryReceivedPacketWithChainHandle;

#[async_trait]
impl<Chain, Counterparty> ReceivedPacketQuerier<Chain, Counterparty>
    for QueryReceivedPacketWithChainHandle
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasBlockingChainHandle,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence>,
{
    async fn query_is_packet_received(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        sequence: &Counterparty::Sequence,
    ) -> Result<bool, Chain::Error> {
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();
        let sequence = *sequence;

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let unreceived_packet = chain_handle
                    .query_unreceived_packets(QueryUnreceivedPacketsRequest {
                        port_id: port_id.clone(),
                        channel_id: channel_id.clone(),
                        packet_commitment_sequences: vec![sequence],
                    })
                    .map_err(Chain::raise_error)?;

                let is_packet_received = unreceived_packet.is_empty();

                Ok(is_packet_received)
            })
            .await
    }
}

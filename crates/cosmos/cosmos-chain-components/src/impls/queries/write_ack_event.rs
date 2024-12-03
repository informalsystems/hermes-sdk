use cgp::prelude::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerier;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use ibc_relayer::event::write_acknowledgement_try_from_abci_event;
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::Height;
use tendermint_rpc::error::Error as TendermintRpcError;
use tendermint_rpc::Client;

use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCosmosWriteAckEvent;

impl<Chain, Counterparty> WriteAckQuerier<Chain, Counterparty> for QueryCosmosWriteAckEvent
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>
        + HasRpcClient
        + CanQueryChainHeight<Height = Height>
        + CanRaiseError<TendermintRpcError>
        + CanRaiseError<&'static str>,
    Counterparty: HasOutgoingPacketType<Chain, OutgoingPacket = Packet>,
{
    async fn query_write_ack_event(
        chain: &Chain,
        packet: &Packet,
    ) -> Result<Option<Chain::WriteAckEvent>, Chain::Error> {
        let rpc_client = chain.rpc_client();

        let latest_height = chain.query_chain_height().await?;

        let block_results = rpc_client
            .block_results(latest_height)
            .await
            .map_err(Chain::raise_error)?;

        let write_ack_event = block_results
            .begin_block_events
            .unwrap_or_default()
            .iter()
            .filter_map(|ev| write_acknowledgement_try_from_abci_event(ev).ok())
            .filter(|ev| &ev.packet == packet)
            .collect::<Vec<WriteAcknowledgement>>()
            .first()
            .cloned();

        // If the WriteAckEvent has not been found in the begin block, look in the block results TXs
        if write_ack_event.is_none() {
            let txs = block_results
                .txs_results
                .ok_or_else(|| Chain::raise_error("block results has empty TXs results"))?;

            for tx in txs {
                let write_ack_event_from_txs = tx
                    .events
                    .iter()
                    .filter_map(|ev| write_acknowledgement_try_from_abci_event(ev).ok())
                    .filter(|ev| &ev.packet == packet)
                    .collect::<Vec<WriteAcknowledgement>>()
                    .first()
                    .cloned();
                if write_ack_event_from_txs.is_some() {
                    return Ok(write_ack_event_from_txs);
                }
            }
        }
        Ok(write_ack_event)
    }
}

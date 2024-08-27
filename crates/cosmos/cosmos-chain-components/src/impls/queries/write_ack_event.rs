use crate::traits::rpc_client::HasRpcClient;
use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerier;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer::chain::cosmos::query::tx::query_packets_from_block;
use ibc_relayer::chain::requests::Qualified;
use ibc_relayer::chain::requests::QueryHeight;
use ibc_relayer::chain::requests::QueryPacketEventDataRequest;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::events::IbcEvent;
use ibc_relayer_types::events::WithBlockDataType;
use ibc_relayer_types::Height;
use tendermint_rpc::{Client, Error as TendermintRpcError};

pub struct QueryWriteAckEventFromAbci;

impl<Chain, Counterparty> WriteAckQuerier<Chain, Counterparty> for QueryWriteAckEventFromAbci
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + HasRpcClient
        + CanRaiseError<TendermintRpcError>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<ibc_relayer::error::Error>,
{
    async fn query_write_ack_event(
        chain: &Chain,
        packet: &Packet,
    ) -> Result<Option<Chain::WriteAckEvent>, Chain::Error> {
        let rpc_client = chain.rpc_client();

        let abci_info = rpc_client.abci_info().await.map_err(Chain::raise_error)?;

        let response = rpc_client
            .header(abci_info.last_block_height)
            .await
            .map_err(Chain::raise_error)?;

        let query_height = Height::new(
            ChainId::chain_version(response.header.chain_id.as_str()),
            u64::from(abci_info.last_block_height),
        )
        .map_err(Chain::raise_error)?;

        let packet = packet.clone();
        let request = QueryPacketEventDataRequest {
            event_id: WithBlockDataType::WriteAck,
            source_channel_id: packet.source_channel.clone(),
            source_port_id: packet.source_port.clone(),
            destination_channel_id: packet.destination_channel.clone(),
            destination_port_id: packet.destination_port.clone(),
            sequences: vec![packet.sequence],
            height: Qualified::Equal(QueryHeight::Specific(query_height)),
        };

        let ibc_events = query_packets_from_block(
            &ChainId::from_string(response.header.chain_id.as_str()),
            rpc_client,
            chain.rpc_address(),
            &request,
        )
        .await
        .map_err(Chain::raise_error)?;

        let write_ack = ibc_events.into_iter().find_map(|event_with_height| {
            let event = event_with_height.event;

            if let IbcEvent::WriteAcknowledgement(write_ack) = event {
                Some(write_ack)
            } else {
                None
            }
        });

        Ok(write_ack)
    }
}

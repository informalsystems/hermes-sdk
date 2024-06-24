use cgp_core::{prelude::*, CanRaiseError, HasErrorType};
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerier;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
// use ibc_relayer::chain::requests::Qualified;
use ibc_relayer::link::packet_events::query_write_ack_events; // could be replaced!
use ibc_relayer::path::PathIdentifiers;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics04_channel::error::Error as Ics04Error;
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
// use ibc_relayer_types::events::IbcEvent;
use crate::traits::grpc_address::HasGrpcAddress;
use crate::traits::rpc_client::HasRpcClient;
use ibc_relayer_types::Height;
use tendermint_rpc::{Client, Error as TendermintRpcError};

pub struct QueryWriteAckEventFromChainHandle;

#[async_trait]
impl<Chain, Counterparty> WriteAckQuerier<Chain, Counterparty> for QueryWriteAckEventFromChainHandle
where
    Chain: HasErrorType
        + HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + HasRpcClient
        + HasGrpcAddress
        + CanRaiseError<TendermintRpcError>
        + CanRaiseError<Ics04Error>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<tonic::transport::Error>,
{
    async fn query_write_ack_event(
        chain: &Chain,
        packet: &Packet,
    ) -> Result<Option<Chain::WriteAckEvent>, Chain::Error> {
        let packet = packet.clone();
        let mut client = ChannelQueryClient::connect(chain.grpc_address().clone())
            .await
            .map_err(|e| Chain::raise_error(e))?;

        // query height
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

        // prepare path identifiers data [port_id, channel_id, counterparty_port_id, counterparty_channel_id]
        let path_ident = PathIdentifiers {
            port_id: packet.destination_port.clone(),
            channel_id: packet.destination_channel.clone(),
            counterparty_port_id: packet.source_port.clone(),
            counterparty_channel_id: packet.source_channel.clone(),
        };

        // chain
        //     .with_blocking_chain_handle(move |chain_handle| {
        //         let status = chain_handle
        //             .query_application_status()
        //             .map_err(Chain::raise_error)?;

        //         let query_height = Qualified::Equal(status.height);

        //         let path_ident = PathIdentifiers {
        //             port_id: packet.destination_port.clone(),
        //             channel_id: packet.destination_channel.clone(),
        //             counterparty_port_id: packet.source_port.clone(),
        //             counterparty_channel_id: packet.source_channel.clone(),
        //         };

        //         let ibc_events = query_write_ack_events(
        //             &chain_handle,
        //             &path_ident,
        //             &[packet.sequence],
        //             query_height,
        //         )
        //         .map_err(Chain::raise_error)?;

        //         let write_ack = ibc_events.into_iter().find_map(|event_with_height| {
        //             let event = event_with_height.event;

        //             if let IbcEvent::WriteAcknowledgement(write_ack) = event {
        //                 Some(write_ack)
        //             } else {
        //                 None
        //             }
        //         });

        //         Ok(write_ack)
        //     })
        //     .await
        Ok(None)
    }
}

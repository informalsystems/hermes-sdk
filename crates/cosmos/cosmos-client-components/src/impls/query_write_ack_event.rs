use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerier;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::Qualified;
use ibc_relayer::link::packet_events::query_write_ack_events;
use ibc_relayer::path::PathIdentifiers;
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::events::IbcEvent;

use crate::traits::chain_handle::HasBlockingChainHandle;

pub struct QueryWriteAckEventFromChainHandle;

#[async_trait]
impl<Chain, Counterparty> WriteAckQuerier<Chain, Counterparty> for QueryWriteAckEventFromChainHandle
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + HasBlockingChainHandle,
{
    async fn query_write_ack_event(
        chain: &Chain,
        packet: &Packet,
    ) -> Result<Option<Chain::WriteAckEvent>, Chain::Error> {
        let packet = packet.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let status = chain_handle
                    .query_application_status()
                    .map_err(Chain::raise_error)?;

                let query_height = Qualified::Equal(status.height);

                let path_ident = PathIdentifiers {
                    port_id: packet.destination_port.clone(),
                    channel_id: packet.destination_channel.clone(),
                    counterparty_port_id: packet.source_port.clone(),
                    counterparty_channel_id: packet.source_channel.clone(),
                };

                let ibc_events = query_write_ack_events(
                    &chain_handle,
                    &path_ident,
                    &[packet.sequence],
                    query_height,
                )
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
            })
            .await
    }
}

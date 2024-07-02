use core::str::FromStr;

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use ibc::core::connection::types::version::Version;
use ibc::core::connection::types::{ConnectionEnd, Counterparty, State as ConnectionState};
use ibc::core::host::types::identifiers::{ClientId, ConnectionId};
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId as RelayerConnectionId;
use ibc_relayer_types::timestamp::ZERO_DURATION;

use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::event::{
    SolomachineConnectionInitEvent, SolomachineCreateClientEvent, SolomachineEvent,
};
use crate::types::message::SolomachineMessage;

pub struct ProcessSolomachineMessages;

impl<Chain> MessageSender<SolomachineChain<Chain>> for ProcessSolomachineMessages
where
    Chain: Solomachine,
{
    async fn send_messages(
        chain: &SolomachineChain<Chain>,
        messages: Vec<SolomachineMessage>,
    ) -> Result<Vec<Vec<SolomachineEvent>>, Chain::Error> {
        let mut res = vec![];
        for message in messages.iter() {
            match message {
                SolomachineMessage::CosmosCreateClient(m) => {
                    let client_id = chain
                        .chain
                        .create_client(m.client_state.clone(), m.consensus_state.clone())
                        .await
                        .unwrap();
                    let create_cient_event = SolomachineCreateClientEvent {
                        client_id,
                        client_state: m.client_state.clone(),
                    };
                    res.push(vec![SolomachineEvent::CreateClient(create_cient_event)]);
                }
                SolomachineMessage::CosmosConnectionOpenInit { .. } => {
                    let connection_id = ConnectionId::from_str("connection-1").unwrap();
                    let counterparty_connection_id =
                        ConnectionId::from_str("connection-0").unwrap();

                    let client_id = ClientId::from_str("cosmos-client").unwrap();
                    let counterparty_client_id = ClientId::from_str("06-solomachine-1").unwrap();

                    let counterparty = Counterparty::new(
                        counterparty_client_id,
                        Some(counterparty_connection_id.clone()),
                        Vec::from("ibc".as_bytes()).try_into().unwrap(),
                    );

                    let connection_end = ConnectionEnd::new(
                        ConnectionState::Init,
                        client_id,
                        counterparty,
                        Version::compatibles(),
                        ZERO_DURATION,
                    )
                    .unwrap();

                    chain
                        .chain
                        .update_connection(&connection_id, connection_end)
                        .await;

                    let connection_init_event = SolomachineConnectionInitEvent {
                        connection_id: RelayerConnectionId::from_str(connection_id.as_str())
                            .unwrap(),
                    };

                    res.push(vec![SolomachineEvent::ConnectionInit(
                        connection_init_event,
                    )]);
                }
                _ => {}
            }
        }

        Ok(res)
    }
}

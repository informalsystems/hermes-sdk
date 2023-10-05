use alloc::sync::Arc;

use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::error::Error;
use crate::types::messages::client::update::CosmosUpdateClientMessage;
use crate::types::payloads::client::CosmosUpdateClientPayload;

pub fn build_update_client_message(
    client_id: &ClientId,
    payload: CosmosUpdateClientPayload,
) -> Result<Vec<Arc<dyn CosmosMessage>>, Error> {
    let messages = payload
        .headers
        .into_iter()
        .map(|header| {
            let message = CosmosUpdateClientMessage {
                client_id: client_id.clone(),
                header: header.into(),
            };

            message.to_cosmos_message()
        })
        .collect();

    Ok(messages)
}

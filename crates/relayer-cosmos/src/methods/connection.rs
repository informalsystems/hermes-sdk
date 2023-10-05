use alloc::sync::Arc;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::contexts::chain::CosmosChain;
use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::connection::CosmosInitConnectionOptions;
use crate::types::error::{BaseError, Error};
use crate::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use crate::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use crate::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use crate::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use crate::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};

pub async fn build_connection_open_init_message<Chain: ChainHandle>(
    chain: &CosmosChain<Chain>,
    client_id: &ClientId,
    counterparty_client_id: &ClientId,
    init_connection_options: &CosmosInitConnectionOptions,
    counterparty_payload: CosmosConnectionOpenInitPayload,
) -> Result<Arc<dyn CosmosMessage>, Error> {
    let client_id = client_id.clone();
    let counterparty_client_id = counterparty_client_id.clone();
    let counterparty_commitment_prefix = counterparty_payload.commitment_prefix;
    let delay_period = init_connection_options.delay_period;

    chain
        .with_blocking_chain_handle(move |chain_handle| {
            let versions = chain_handle
                .query_compatible_versions()
                .map_err(BaseError::relayer)?;

            let version = versions.into_iter().next().unwrap_or_default();

            let message = CosmosConnectionOpenInitMessage {
                client_id,
                counterparty_client_id,
                counterparty_commitment_prefix,
                version,
                delay_period,
            };

            Ok(message.to_cosmos_message())
        })
        .await
}

pub fn build_connection_open_try_message(
    client_id: &ClientId,
    counterparty_client_id: &ClientId,
    counterparty_connection_id: &ConnectionId,
    counterparty_payload: CosmosConnectionOpenTryPayload,
) -> Result<Arc<dyn CosmosMessage>, Error> {
    let message = CosmosConnectionOpenTryMessage {
        client_id: client_id.clone(),
        counterparty_client_id: counterparty_client_id.clone(),
        counterparty_connection_id: counterparty_connection_id.clone(),
        counterparty_commitment_prefix: counterparty_payload.commitment_prefix.clone(),
        counterparty_versions: counterparty_payload.versions,
        delay_period: counterparty_payload.delay_period,
        client_state: counterparty_payload.client_state.into(),
        update_height: counterparty_payload.update_height,
        proof_init: counterparty_payload.proof_init,
        proof_client: counterparty_payload.proof_client,
        proof_consensus: counterparty_payload.proof_consensus,
    };

    Ok(message.to_cosmos_message())
}

pub fn build_connection_open_ack_message(
    connection_id: &ConnectionId,
    counterparty_connection_id: &ConnectionId,
    counterparty_payload: CosmosConnectionOpenAckPayload,
) -> Result<Arc<dyn CosmosMessage>, Error> {
    let connection_id = connection_id.clone();
    let counterparty_connection_id = counterparty_connection_id.clone();

    let message = CosmosConnectionOpenAckMessage {
        connection_id,
        counterparty_connection_id,
        version: counterparty_payload.version,
        client_state: counterparty_payload.client_state.into(),
        update_height: counterparty_payload.update_height,
        proof_try: counterparty_payload.proof_try,
        proof_client: counterparty_payload.proof_client,
        proof_consensus: counterparty_payload.proof_consensus,
    };

    Ok(message.to_cosmos_message())
}

pub fn build_connection_open_confirm_message(
    connection_id: &ConnectionId,
    counterparty_payload: CosmosConnectionOpenConfirmPayload,
) -> Result<Arc<dyn CosmosMessage>, Error> {
    let message = CosmosConnectionOpenConfirmMessage {
        connection_id: connection_id.clone(),
        update_height: counterparty_payload.update_height,
        proof_ack: counterparty_payload.proof_ack,
    };

    Ok(message.to_cosmos_message())
}

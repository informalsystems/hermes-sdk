use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenAckMessageBuilderComponent,
    ConnectionOpenConfirmMessageBuilder, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilder, ConnectionOpenInitMessageBuilderComponent,
    ConnectionOpenTryMessageBuilder, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::host::types::identifiers::{ClientId, ConnectionId};
use ibc_proto::google::protobuf::Any as IbcProtoAny;

use crate::methods::encode::sign_data::timestamped_sign_data_to_bytes;
use crate::types::payloads::connection::{
    SolomachineConnectionOpenAckPayload, SolomachineConnectionOpenConfirmPayload,
    SolomachineConnectionOpenInitPayload, SolomachineConnectionOpenTryPayload,
};

pub struct BuildSolomachineConnectionHandshakeMessagesForCosmos;

#[cgp_provider(ConnectionOpenInitMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakeMessagesForCosmos
where
    Chain: HasInitConnectionOptionsType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasAsyncErrorType,
    Counterparty: HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        _client_id: &ClientId,
        _counterparty_client_id: &ClientId,
        _init_connection_options: &Chain::InitConnectionOptions,
        _counterparty_payload: SolomachineConnectionOpenInitPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }
}

#[cgp_provider(ConnectionOpenTryMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakeMessagesForCosmos
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasAsyncErrorType,
    Counterparty: HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = SolomachineConnectionOpenTryPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_try_message(
        _chain: &Chain,
        client_id: &ClientId,
        counterparty_client_id: &ClientId,
        counterparty_connection_id: &ConnectionId,
        payload: SolomachineConnectionOpenTryPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let counterparty_commitment_prefix = payload.commitment_prefix;

        let proof_init = timestamped_sign_data_to_bytes(&payload.proof_init);

        let proof_client = timestamped_sign_data_to_bytes(&payload.proof_client);

        let consensus_signature = timestamped_sign_data_to_bytes(&payload.proof_consensus);

        let client_state_any = IbcProtoAny::from(payload.client_state);

        let message = CosmosConnectionOpenTryMessage {
            client_id: client_id.to_string(),
            counterparty_client_id: counterparty_client_id.to_string(),
            counterparty_connection_id: counterparty_connection_id.to_string(),
            counterparty_commitment_prefix,
            counterparty_versions: payload.versions,
            delay_period: payload.delay_period,
            client_state: Any {
                type_url: client_state_any.type_url,
                value: client_state_any.value,
            },
            update_height: payload.update_height,
            proof_init,
            proof_client,
            proof_consensus: consensus_signature,
            proof_consensus_height: payload.update_height,
        };

        Ok(message.to_cosmos_message())
    }
}

#[cgp_provider(ConnectionOpenAckMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakeMessagesForCosmos
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasAsyncErrorType,
    Counterparty: HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = SolomachineConnectionOpenAckPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_ack_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_connection_id: &ConnectionId,
        _counterparty_payload: SolomachineConnectionOpenAckPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }
}

#[cgp_provider(ConnectionOpenConfirmMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakeMessagesForCosmos
where
    Chain: HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasAsyncErrorType,
    Counterparty: HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = SolomachineConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_payload: SolomachineConnectionOpenConfirmPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }
}

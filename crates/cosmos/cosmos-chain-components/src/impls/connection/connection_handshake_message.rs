use core::fmt::Display;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasMessageType;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::types::AsBytes;
use hermes_relayer_components::chain::traits::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenAckMessageBuilderComponent,
    ConnectionOpenConfirmMessageBuilder, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilder, ConnectionOpenInitMessageBuilderComponent,
    ConnectionOpenTryMessageBuilder, ConnectionOpenTryMessageBuilderComponent, HasClientIdType,
    HasClientStateType, HasCommitmentPrefixType, HasCommitmentProofBytes, HasConnectionEndType,
    HasConnectionIdType, HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType, HasHeightFields,
    HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::connection::types::ConnectionEnd;
use ibc_proto::ibc::core::connection::v1::Version;
use prost_types::Any;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::connection::CosmosInitConnectionOptions;
use crate::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use crate::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use crate::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use crate::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;

pub struct BuildCosmosConnectionHandshakeMessage;

#[cgp_provider(ConnectionOpenInitMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasMessageType
        + HasClientIdType<Counterparty, ClientId: Display>
        + HasAsyncErrorType,
    Counterparty: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasClientIdType<Chain, ClientId: Display>
        + HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload<Counterparty>,
        >,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &CosmosInitConnectionOptions,
        counterparty_payload: ConnectionOpenInitPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let counterparty_commitment_prefix = counterparty_payload.commitment_prefix;
        let delay_period = init_connection_options.delay_period;

        let version = default_connection_version();

        let message = CosmosConnectionOpenInitMessage {
            client_id: client_id.to_string(),
            counterparty_client_id: counterparty_client_id.to_string(),
            counterparty_commitment_prefix,
            version,
            delay_period,
        };

        Ok(message.to_cosmos_message().into())
    }
}

#[cgp_provider(ConnectionOpenTryMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasHeightFields
        + HasMessageType
        + HasClientIdType<Counterparty, ClientId: Display>
        + HasClientStateType<Counterparty>
        + HasEncoding<AsBytes, Encoding = Encoding>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofBytes
        + HasHeightFields
        + HasClientIdType<Chain, ClientId: Display>
        + HasConnectionIdType<Chain, ConnectionId: Display>
        + HasConnectionEndType<Chain, ConnectionEnd = ConnectionEnd>
        + HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload<Counterparty, Chain>,
        >,
    Encoding: CanConvert<Chain::ClientState, Any>,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_connection_open_try_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        payload: ConnectionOpenTryPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let counterparty_versions = payload.connection_end.versions().to_vec();
        let delay_period = payload.connection_end.delay_period();

        let client_state_any = chain
            .encoding()
            .convert(&payload.client_state)
            .map_err(Chain::raise_error)?;

        let proof_consensus_height = Height::new(
            Chain::revision_number(&payload.proof_consensus_height),
            Chain::revision_height(&payload.proof_consensus_height),
        )
        .map_err(Chain::raise_error)?;

        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let proof_init = Counterparty::commitment_proof_bytes(&payload.proof_init).into();
        let proof_client = Counterparty::commitment_proof_bytes(&payload.proof_client).into();
        let proof_consensus = Counterparty::commitment_proof_bytes(&payload.proof_consensus).into();

        let message = CosmosConnectionOpenTryMessage {
            client_id: client_id.to_string(),
            counterparty_client_id: counterparty_client_id.to_string(),
            counterparty_connection_id: counterparty_connection_id.to_string(),
            counterparty_commitment_prefix: payload.commitment_prefix,
            counterparty_versions,
            delay_period,
            client_state: client_state_any,
            update_height,
            proof_init,
            proof_client,
            proof_consensus,
            proof_consensus_height,
        };

        Ok(message.to_cosmos_message().into())
    }
}

#[cgp_provider(ConnectionOpenAckMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasMessageType
        + HasHeightFields
        + HasClientStateType<Counterparty>
        + HasClientIdType<Counterparty, ClientId: Display>
        + HasConnectionIdType<Counterparty, ConnectionId: Display>
        + HasEncoding<AsBytes, Encoding = Encoding>
        + CanRaiseAsyncError<Encoding::Error>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<&'static str>,
    Counterparty: HasCommitmentProofBytes
        + HasConnectionEndType<Chain, ConnectionEnd = ConnectionEnd>
        + HasClientIdType<Chain, ClientId: Display>
        + HasConnectionIdType<Chain, ConnectionId: Display>
        + HasHeightFields
        + HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload<Counterparty, Chain>,
        >,
    Encoding: CanConvert<Chain::ClientState, Any>,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_connection_open_ack_message(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        payload: ConnectionOpenAckPayload<Counterparty, Chain>,
    ) -> Result<Chain::Message, Chain::Error> {
        let version = payload
            .connection_end
            .versions()
            .iter()
            .next()
            .cloned()
            .map(Version::from)
            .unwrap_or_else(default_connection_version);

        let client_state_any = chain
            .encoding()
            .convert(&payload.client_state)
            .map_err(Chain::raise_error)?;

        let proof_consensus_height = Height::new(
            Chain::revision_number(&payload.proof_consensus_height),
            Chain::revision_height(&payload.proof_consensus_height),
        )
        .map_err(Chain::raise_error)?;

        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let proof_try = Counterparty::commitment_proof_bytes(&payload.proof_try).into();
        let proof_client = Counterparty::commitment_proof_bytes(&payload.proof_client).into();
        let proof_consensus = Counterparty::commitment_proof_bytes(&payload.proof_consensus).into();

        let message = CosmosConnectionOpenAckMessage {
            connection_id: connection_id.to_string(),
            counterparty_connection_id: counterparty_connection_id.to_string(),
            version,
            client_state: client_state_any,
            update_height,
            proof_try,
            proof_client,
            proof_consensus,
            proof_consensus_height,
        };

        Ok(message.to_cosmos_message().into())
    }
}

#[cgp_provider(ConnectionOpenConfirmMessageBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasMessageType
        + HasConnectionIdType<Counterparty, ConnectionId: Display>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasCommitmentProofBytes
        + HasHeightFields
        + HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Counterparty>,
        >,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        payload: ConnectionOpenConfirmPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let update_height = Height::new(
            Counterparty::revision_number(&payload.update_height),
            Counterparty::revision_height(&payload.update_height),
        )
        .map_err(Chain::raise_error)?;

        let proof_ack = Counterparty::commitment_proof_bytes(&payload.proof_ack).into();

        let message = CosmosConnectionOpenConfirmMessage {
            connection_id: connection_id.to_string(),
            update_height,
            proof_ack,
        };

        Ok(message.to_cosmos_message().into())
    }
}

pub fn default_connection_version() -> Version {
    Version {
        identifier: "1".to_string(),
        features: vec!["ORDER_ORDERED".to_string(), "ORDER_UNORDERED".to_string()],
    }
}

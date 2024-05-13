use cgp_core::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_relayer_components::chain::traits::commitment_prefix::HasCommitmentPrefixType;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenConfirmMessageBuilder,
    ConnectionOpenInitMessageBuilder, ConnectionOpenTryMessageBuilder,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionEndType, HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::types::connection_payload::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics03_connection::connection::ConnectionEnd;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use prost_types::Any;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::connection::CosmosInitConnectionOptions;
use crate::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use crate::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use crate::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use crate::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;

pub struct BuildCosmosConnectionHandshakeMessage;

impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload<Chain>,
        >,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: ConnectionOpenInitPayload<Chain>,
    ) -> Result<CosmosMessage, Chain::Error> {
        let client_id = client_id.clone();
        let counterparty_client_id = counterparty_client_id.clone();
        let counterparty_commitment_prefix = counterparty_payload.commitment_prefix;
        let delay_period = init_connection_options.delay_period;

        let version = Version::default();

        let message = CosmosConnectionOpenInitMessage {
            client_id,
            counterparty_client_id,
            counterparty_commitment_prefix,
            version,
            delay_period,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty, Encoding> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Height = Height,
            Message = CosmosMessage,
        > + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId, Height = Height>
        + HasClientStateType<Chain>
        + HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload<Chain, Counterparty>,
        > + HasDefaultEncoding<Encoding = Encoding>,
    Encoding: CanConvert<Counterparty::ClientState, Any>,
{
    async fn build_connection_open_try_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        payload: ConnectionOpenTryPayload<Chain, Counterparty>,
    ) -> Result<CosmosMessage, Chain::Error> {
        let counterparty_versions = payload.connection_end.versions().to_vec();
        let delay_period = payload.connection_end.delay_period();

        let client_state_any = Counterparty::default_encoding()
            .convert(&payload.client_state)
            .map_err(Chain::raise_error)?;

        let message = CosmosConnectionOpenTryMessage {
            client_id: client_id.clone(),
            counterparty_client_id: counterparty_client_id.clone(),
            counterparty_connection_id: counterparty_connection_id.clone(),
            counterparty_commitment_prefix: payload.commitment_prefix,
            counterparty_versions,
            delay_period,
            client_state: client_state_any,
            update_height: payload.update_height,
            proof_init: payload.proof_init,
            proof_client: payload.proof_client,
            proof_consensus: payload.proof_consensus,
            proof_consensus_height: payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty, Encoding> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
            Height = Height,
        > + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId, Height = Height>
        + HasClientStateType<Chain>
        + HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload<Chain, Counterparty>,
        > + HasDefaultEncoding<Encoding = Encoding>,
    Encoding: CanConvert<Counterparty::ClientState, Any>,
{
    async fn build_connection_open_ack_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: ConnectionOpenAckPayload<Chain, Counterparty>,
    ) -> Result<CosmosMessage, Chain::Error> {
        let connection_id = connection_id.clone();
        let counterparty_connection_id = counterparty_connection_id.clone();

        let version = counterparty_payload
            .connection_end
            .versions()
            .iter()
            .next()
            .cloned()
            .unwrap_or_default();

        let client_state_any = Counterparty::default_encoding()
            .convert(&counterparty_payload.client_state)
            .map_err(Chain::raise_error)?;

        let message = CosmosConnectionOpenAckMessage {
            connection_id,
            counterparty_connection_id,
            version,
            client_state: client_state_any,
            update_height: counterparty_payload.update_height,
            proof_try: counterparty_payload.proof_try,
            proof_client: counterparty_payload.proof_client,
            proof_consensus: counterparty_payload.proof_consensus,
            proof_consensus_height: counterparty_payload.proof_consensus_height,
        };

        Ok(message.to_cosmos_message())
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasIbcChainTypes<
            Counterparty,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
            Height = Height,
        > + HasErrorType,
    Counterparty: HasConnectionOpenConfirmPayloadType<
        Chain,
        ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Chain>,
    >,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_payload: ConnectionOpenConfirmPayload<Chain>,
    ) -> Result<CosmosMessage, Chain::Error> {
        let message = CosmosConnectionOpenConfirmMessage {
            connection_id: connection_id.clone(),
            update_height: counterparty_payload.update_height,
            proof_ack: counterparty_payload.proof_ack,
        };

        Ok(message.to_cosmos_message())
    }
}

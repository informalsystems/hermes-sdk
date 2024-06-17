use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
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
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofBytes;
use hermes_relayer_components::chain::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};
use ibc::core::connection::types::version::Version;
use ibc::core::connection::types::ConnectionEnd;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics02_client::height::Height;
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
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = CosmosInitConnectionOptions,
        > + HasIbcChainTypes<Counterparty, ClientId = ClientId, ConnectionId = ConnectionId>
        + CanRaiseError<&'static str>,
    Counterparty: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
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
        init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: ConnectionOpenInitPayload<Counterparty>,
    ) -> Result<Chain::Message, Chain::Error> {
        let client_id = client_id.clone();
        let counterparty_client_id = counterparty_client_id.clone();
        let counterparty_commitment_prefix = counterparty_payload.commitment_prefix;
        let delay_period = init_connection_options.delay_period;

        let version = Version::compatibles()
            .into_iter()
            .next()
            .ok_or_else(|| Chain::raise_error("expect default version to be present"))?;

        let message = CosmosConnectionOpenInitMessage {
            client_id,
            counterparty_client_id,
            counterparty_commitment_prefix,
            version,
            delay_period,
        };

        Ok(message.to_cosmos_message().into())
    }
}

impl<Chain, Counterparty, Encoding> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasHeightFields
        + HasClientStateType<Counterparty>
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofBytes
        + HasHeightFields
        + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
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
            client_id: client_id.clone(),
            counterparty_client_id: counterparty_client_id.clone(),
            counterparty_connection_id: counterparty_connection_id.clone(),
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

impl<Chain, Counterparty, Encoding> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasClientStateType<Counterparty>
        + HasHeightFields
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<&'static str>,
    Counterparty: HasCommitmentProofBytes
        + HasConnectionEndType<Chain, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
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
        let connection_id = connection_id.clone();
        let counterparty_connection_id = counterparty_connection_id.clone();

        let version = payload
            .connection_end
            .versions()
            .iter()
            .next()
            .cloned()
            .or_else(|| Version::compatibles().into_iter().next())
            .ok_or_else(|| Chain::raise_error("expect default version to be present"))?;

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
            connection_id,
            counterparty_connection_id,
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

impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<Counterparty, ConnectionId = ConnectionId> + CanRaiseError<Ics02Error>,
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
            connection_id: connection_id.clone(),
            update_height,
            proof_ack,
        };

        Ok(message.to_cosmos_message().into())
    }
}

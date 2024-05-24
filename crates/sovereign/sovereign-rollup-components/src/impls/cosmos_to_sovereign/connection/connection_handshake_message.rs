use cgp_core::{CanRaiseError, HasErrorType};
use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_protobuf_encoding_components::types::Any;
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
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};
use ibc::core::connection::types::version::Version;
use ibc::core::connection::types::ConnectionEnd;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::types::client_state::WrappedSovereignClientState;
use crate::types::height::RollupHeight;
use crate::types::message::SovereignMessage;
use crate::types::payloads::connection::SovereignInitConnectionOptions;

pub struct BuildCosmosConnectionHandshakeMessageOnSovereign;

impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessageOnSovereign
where
    Chain: HasInitConnectionOptionsType<
            Counterparty,
            InitConnectionOptions = SovereignInitConnectionOptions,
        > + HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasErrorType,
    Counterparty: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload<Counterparty>,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        client_id: &ClientId,
        counterparty_client_id: &ClientId,
        init_connection_options: &SovereignInitConnectionOptions,
        counterparty_payload: ConnectionOpenInitPayload<Counterparty>,
    ) -> Result<SovereignMessage, Chain::Error> {
        let commitment_prefix = counterparty_payload.commitment_prefix;

        let SovereignInitConnectionOptions {
            delay_period,
            connection_version,
        } = init_connection_options;

        let msg = CosmosConnectionOpenInitMessage {
            client_id: client_id.to_owned(),
            counterparty_client_id: counterparty_client_id.to_owned(),
            counterparty_commitment_prefix: commitment_prefix,
            version: connection_version.to_owned(),
            delay_period: delay_period.to_owned(),
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}

impl<Chain, Counterparty, Encoding> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessageOnSovereign
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Height = RollupHeight,
        > + HasClientStateType<Counterparty, ClientState = WrappedSovereignClientState>
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasConnectionEndType<Chain, ConnectionEnd = ConnectionEnd>
        + HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload<Counterparty, Chain>,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId, Height = Height>,
    Encoding: CanConvert<WrappedSovereignClientState, Any>,
{
    async fn build_connection_open_try_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        payload: ConnectionOpenTryPayload<Counterparty, Chain>,
    ) -> Result<SovereignMessage, Chain::Error> {
        let counterparty_versions = payload.connection_end.versions().to_vec();
        let delay_period = payload.connection_end.delay_period();

        let client_state_any = chain
            .encoding()
            .convert(&payload.client_state)
            .map_err(Chain::raise_error)?;

        let proof_consensus_height = Height::new(0, payload.proof_consensus_height.slot_number)
            .map_err(Chain::raise_error)?;

        let msg = CosmosConnectionOpenTryMessage {
            client_id: client_id.to_owned(),
            counterparty_client_id: counterparty_client_id.to_owned(),
            counterparty_connection_id: counterparty_connection_id.to_owned(),
            counterparty_commitment_prefix: payload.commitment_prefix,
            counterparty_versions,
            client_state: client_state_any,
            delay_period,
            update_height: payload.update_height,
            proof_init: payload.proof_init,
            proof_client: payload.proof_client,
            proof_consensus: payload.proof_consensus,
            proof_consensus_height,
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}

impl<Chain, Counterparty, Encoding> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessageOnSovereign
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Height = RollupHeight,
        > + HasClientStateType<Counterparty, ClientState = WrappedSovereignClientState>
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<&'static str>,
    Counterparty: HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasConnectionEndType<Chain, ConnectionEnd = ConnectionEnd>
        + HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload<Counterparty, Chain>,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId, Height = Height>,
    Encoding: CanConvert<WrappedSovereignClientState, Any>,
{
    async fn build_connection_open_ack_message(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        payload: ConnectionOpenAckPayload<Counterparty, Chain>,
    ) -> Result<SovereignMessage, Chain::Error> {
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

        let proof_consensus_height = Height::new(0, payload.proof_consensus_height.slot_number)
            .map_err(Chain::raise_error)?;

        let msg = CosmosConnectionOpenAckMessage {
            connection_id: connection_id.to_owned(),
            counterparty_connection_id: counterparty_connection_id.to_owned(),
            client_state: client_state_any,
            version,
            update_height: payload.update_height,
            proof_try: payload.proof_try,
            proof_client: payload.proof_client,
            proof_consensus: payload.proof_consensus,
            proof_consensus_height,
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessageOnSovereign
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasErrorType,
    Counterparty: HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + HasHeightType<Height = Height>
        + HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Counterparty>,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_payload: ConnectionOpenConfirmPayload<Counterparty>,
    ) -> Result<SovereignMessage, Chain::Error> {
        let msg = CosmosConnectionOpenConfirmMessage {
            connection_id: connection_id.to_owned(),
            update_height: counterparty_payload.update_height,
            proof_ack: counterparty_payload.proof_ack,
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}

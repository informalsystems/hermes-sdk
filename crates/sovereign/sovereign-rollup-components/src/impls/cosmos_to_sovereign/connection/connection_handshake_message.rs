use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloadTypes, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::google::protobuf::Any;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use prost::{Message, Name};

use crate::types::message::SovereignMessage;
use crate::types::messages::ibc::IbcMessage;
use crate::types::payloads::connection::SovereignInitConnectionOptions;

pub struct BuildCosmosConnectionHandshakeMessageOnSovereign;

impl<Chain, Counterparty> ConnectionHandshakeMessageBuilder<Chain, Counterparty>
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
        > + HasDefaultSigner<Signer = Signer>
        + HasErrorType,
    Counterparty: HasConnectionHandshakePayloadTypes<
            Chain,
            ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload,
            ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload,
            ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        _client_id: &Chain::ClientId,
        _counterparty_client_id: &Counterparty::ClientId,
        _init_connection_options: &Chain::InitConnectionOptions,
        _counterparty_payload: CosmosConnectionOpenInitPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_try_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: CosmosConnectionOpenTryPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        // TODO(rano): finish connection open try message

        let CosmosConnectionOpenTryPayload {
            commitment_prefix,
            client_state,
            versions,
            delay_period,
            update_height,
            proof_init,
            proof_client,
            proof_consensus,
        } = counterparty_payload;

        let msg = CosmosConnectionOpenTryMessage {
            client_id: client_id.to_owned(),
            counterparty_client_id: counterparty_client_id.to_owned(),
            counterparty_connection_id: counterparty_connection_id.to_owned(),
            counterparty_commitment_prefix: commitment_prefix,
            counterparty_versions: versions,
            client_state: client_state.into(),
            delay_period,
            update_height,
            proof_init,
            proof_client,
            proof_consensus,
        };

        let msg_any = msg.encode_protobuf(chain.get_default_signer());

        Ok(SovereignMessage::Ibc(IbcMessage::Core(msg_any)))
    }

    async fn build_connection_open_ack_message(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _counterparty_connection_id: &Counterparty::ConnectionId,
        _counterparty_payload: CosmosConnectionOpenAckPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _counterparty_payload: CosmosConnectionOpenConfirmPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }
}

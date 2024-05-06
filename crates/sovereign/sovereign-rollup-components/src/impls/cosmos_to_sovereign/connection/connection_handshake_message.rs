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
        > + HasErrorType,
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
        _chain: &Chain,
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

        let counterparty = ibc_proto::ibc::core::connection::v1::Counterparty {
            client_id: counterparty_client_id.to_string(),
            connection_id: counterparty_connection_id.to_string(),
            prefix: Some(ibc_proto::ibc::core::commitment::v1::MerklePrefix {
                key_prefix: commitment_prefix.into_vec(),
            }),
        };

        let msg = ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry {
            client_id: client_id.to_string(),
            client_state: Some(client_state.into()),
            counterparty: Some(counterparty),
            delay_period: delay_period.as_secs(),
            counterparty_versions: versions.into_iter().map(Into::into).collect(),
            proof_height: Some(update_height.into()),
            proof_init: proof_init.into(),
            proof_client: proof_client.into(),
            proof_consensus: proof_consensus.proof().clone().into_bytes(),
            consensus_height: Some(proof_consensus.height().into()),
            signer: "signer".into(),

            // optional: needed when ibc module can't query host consensus state.
            host_consensus_state_proof: vec![],

            // deprecated fields
            previous_connection_id: "".into(),
        };

        let msg_any = Any {
            type_url: ibc_proto::ibc::core::connection::v1::MsgConnectionOpenTry::full_name(),
            value: msg.encode_to_vec(),
        };

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

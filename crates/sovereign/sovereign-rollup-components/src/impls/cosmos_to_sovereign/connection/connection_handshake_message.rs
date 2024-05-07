use cgp_core::HasErrorType;

use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::traits::message::DynCosmosMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_ack::CosmosConnectionOpenAckMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_confirm::CosmosConnectionOpenConfirmMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_init::CosmosConnectionOpenInitMessage;
use hermes_cosmos_chain_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use hermes_cosmos_chain_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloadTypes, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::signer::Signer;

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
        > + HasBlockingChainHandle,
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
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: CosmosConnectionOpenInitPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        let CosmosConnectionOpenInitPayload { commitment_prefix } = counterparty_payload;

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

        let msg_any = msg.encode_protobuf(&Signer::dummy());

        Ok(SovereignMessage::Ibc(IbcMessage::Core(msg_any)))
    }

    async fn build_connection_open_try_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: CosmosConnectionOpenTryPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
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

        let msg_any = msg.encode_protobuf(&Signer::dummy());

        Ok(SovereignMessage::Ibc(IbcMessage::Core(msg_any)))
    }

    async fn build_connection_open_ack_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: CosmosConnectionOpenAckPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        let CosmosConnectionOpenAckPayload {
            client_state,
            version,
            update_height,
            proof_try,
            proof_client,
            proof_consensus,
        } = counterparty_payload;

        let msg = CosmosConnectionOpenAckMessage {
            connection_id: connection_id.to_owned(),
            counterparty_connection_id: counterparty_connection_id.to_owned(),
            client_state: client_state.into(),
            version,
            update_height,
            proof_try,
            proof_client,
            proof_consensus,
        };

        let msg_any = msg.encode_protobuf(&Signer::dummy());

        Ok(SovereignMessage::Ibc(IbcMessage::Core(msg_any)))
    }

    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_payload: CosmosConnectionOpenConfirmPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        let CosmosConnectionOpenConfirmPayload {
            update_height,
            proof_ack,
        } = counterparty_payload;

        let msg = CosmosConnectionOpenConfirmMessage {
            connection_id: connection_id.to_owned(),
            update_height,
            proof_ack,
        };

        let msg_any = msg.encode_protobuf(&Signer::dummy());

        Ok(SovereignMessage::Ibc(IbcMessage::Core(msg_any)))
    }
}

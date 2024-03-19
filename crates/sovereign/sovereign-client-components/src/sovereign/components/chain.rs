use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelHandshakePayloadTypeComponent, InitChannelOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionHandshakePayloadTypeComponent, InitConnectionOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientOptionsTypeComponent, CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;
use hermes_relayer_components::transaction::traits::types::{
    fee::FeeTypeComponent, nonce::NonceTypeComponent, signer::SignerTypeComponent,
    transaction::TransactionTypeComponent, tx_hash::TransactionHashTypeComponent,
    tx_response::TxResponseTypeComponent,
};

use crate::sovereign::impls::client::create_client_message::BuildCreateCosmosClientMessageOnSovereign;
use crate::sovereign::impls::client::create_client_payload::BuildSovereignCreateClientPayload;
use crate::sovereign::impls::client::update_client_message::BuildUpdateCosmosClientMessageOnSovereign;
use crate::sovereign::impls::client::update_client_payload::BuildSovereignUpdateClientPayload;
use crate::sovereign::impls::connection::connection_handshake_payload::BuildSovereignConnectionHandshakePayload;
use crate::sovereign::impls::types::chain::ProvideSovereignChainTypes;
use crate::sovereign::impls::types::client_state::ProvideSovereignClientState;
use crate::sovereign::impls::types::payload::ProvideSovereignPayloadTypes;
use crate::sovereign::impls::types::transaction::ProvideSovereignTransactionTypes;

pub struct SovereignChainClientComponents;

delegate_components! {
    #[mark_component(IsSovereignChainClientComponent)]
    SovereignChainClientComponents {
        [
            HeightTypeComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
        ]:
            ProvideSovereignChainTypes,
        [
            CreateClientOptionsTypeComponent,
            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            InitConnectionOptionsTypeComponent,
            ConnectionHandshakePayloadTypeComponent,
            InitChannelOptionsTypeComponent,
            ChannelHandshakePayloadTypeComponent,
        ]:
            ProvideSovereignPayloadTypes,
        ClientStateTypeComponent:
            ProvideSovereignClientState,
        [
            TransactionTypeComponent,
            NonceTypeComponent,
            FeeTypeComponent,
            SignerTypeComponent,
            TransactionHashTypeComponent,
            TxResponseTypeComponent,
        ]:
            ProvideSovereignTransactionTypes,
        CreateClientPayloadBuilderComponent:
            BuildSovereignCreateClientPayload,
        CreateClientMessageBuilderComponent:
            BuildCreateCosmosClientMessageOnSovereign,
        UpdateClientPayloadBuilderComponent:
            BuildSovereignUpdateClientPayload,
        UpdateClientMessageBuilderComponent:
            BuildUpdateCosmosClientMessageOnSovereign,
        ConnectionHandshakePayloadBuilderComponent:
            BuildSovereignConnectionHandshakePayload,
    }
}

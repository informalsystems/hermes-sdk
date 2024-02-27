use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelHandshakePayloadTypeComponent, InitChannelOptionsTypeComponent,
};
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
use hermes_relayer_components::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::types::{
    FeeTypeComponent, NonceTypeComponent, SignerTypeComponent, TransactionHashTypeComponent,
    TransactionTypeComponent, TxResponseTypeComponent,
};

use crate::sovereign::impls::client::create_client_message::BuildCreateCosmosClientMessageOnSovereign;
use crate::sovereign::impls::client::create_client_payload::BuildSovereignCreateClientPayload;
use crate::sovereign::impls::client::update_client_message::BuildUpdateCosmosClientMessageOnSovereign;
use crate::sovereign::impls::client::update_client_payload::BuildSovereignUpdateClientPayload;
use crate::sovereign::impls::rpc::json_rpc_client::ProvideJsonRpseeClient;
use crate::sovereign::impls::transaction::publish_batch::PublishSovereignTransactionBatch;
use crate::sovereign::impls::transaction::query_tx_response::QuerySovereignTxResponse;
use crate::sovereign::impls::types::chain::ProvideSovereignChainTypes;
use crate::sovereign::impls::types::payload::ProvideSovereignPayloadTypes;
use crate::sovereign::impls::types::transaction::ProvideSovereignTransactionTypes;
use crate::sovereign::traits::rollup::json_rpc_client::JsonRpcClientTypeComponent;
use crate::sovereign::traits::rollup::publish_batch::TransactionBatchPublisherComponent;

pub struct SovereignClientComponents;

delegate_components! {
    #[mark_component(IsSovereignClientComponent)]
    SovereignClientComponents {
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
        JsonRpcClientTypeComponent:
            ProvideJsonRpseeClient,
        TransactionBatchPublisherComponent:
            PublishSovereignTransactionBatch,
        TxResponseQuerierComponent:
            QuerySovereignTxResponse,
    }
}

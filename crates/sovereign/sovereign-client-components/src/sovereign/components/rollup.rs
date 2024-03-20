use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::transaction::poll_timeout::DefaultPollTimeout;
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
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    PollTimeoutGetterComponent, PollTxResponse,
};
use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::poll_tx_response::TxResponsePollerComponent;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;

use crate::sovereign::impls::rpc::json_rpc_client::ProvideJsonRpseeClient;
use crate::sovereign::impls::transaction::event::ParseSovTxResponseAsEvents;
use crate::sovereign::impls::transaction::publish_batch::PublishSovereignTransactionBatch;
use crate::sovereign::impls::transaction::query_tx_response::QuerySovereignTxResponse;
use crate::sovereign::impls::types::chain::ProvideSovereignChainTypes;
use crate::sovereign::impls::types::payload::ProvideSovereignPayloadTypes;
use crate::sovereign::impls::types::transaction::ProvideSovereignTransactionTypes;
use crate::sovereign::traits::rollup::json_rpc_client::JsonRpcClientTypeComponent;
use crate::sovereign::traits::rollup::publish_batch::TransactionBatchPublisherComponent;

pub struct SovereignRollupClientComponents;

delegate_components! {
    #[mark_component(IsSovereignRollupClientComponent)]
    SovereignRollupClientComponents {
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
        JsonRpcClientTypeComponent:
            ProvideJsonRpseeClient,
        TransactionBatchPublisherComponent:
            PublishSovereignTransactionBatch,
        TxResponseQuerierComponent:
            QuerySovereignTxResponse,
        TxResponsePollerComponent:
            PollTxResponse,
        PollTimeoutGetterComponent:
            DefaultPollTimeout,
        TxResponseAsEventsParserComponent:
            ParseSovTxResponseAsEvents,
    }
}

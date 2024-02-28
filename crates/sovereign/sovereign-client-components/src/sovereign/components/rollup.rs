use cgp_core::prelude::*;
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

use crate::sovereign::impls::queries::events::QuerySovereignEvents;
use crate::sovereign::impls::rpc::json_rpc_client::ProvideJsonRpseeClient;
use crate::sovereign::impls::transaction::publish_batch::PublishSovereignTransactionBatch;
use crate::sovereign::impls::transaction::query_tx_response::QuerySovereignTxResponse;
use crate::sovereign::impls::types::chain::ProvideSovereignChainTypes;
use crate::sovereign::impls::types::payload::ProvideSovereignPayloadTypes;
use crate::sovereign::impls::types::transaction::ProvideSovereignTransactionTypes;
use crate::sovereign::traits::rollup::json_rpc_client::JsonRpcClientTypeComponent;
use crate::sovereign::traits::rollup::publish_batch::TransactionBatchPublisherComponent;
use crate::sovereign::traits::rollup::queries::events::EventsByEventIdsQuerierComponent;
use crate::sovereign::traits::rollup::types::event_id::EventIdTypeComponent;

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
            EventIdTypeComponent,
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
        EventsByEventIdsQuerierComponent:
            QuerySovereignEvents,
    }
}

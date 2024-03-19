use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::transaction::components::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::traits::components::nonce_querier::NonceQuerierComponent;
use hermes_relayer_components::transaction::traits::components::tx_encoder::TxEncoderComponent;
use hermes_relayer_components::transaction::traits::components::tx_fee_estimater::TxFeeEstimatorComponent;
use hermes_relayer_components::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::components::tx_submitter::TxSubmitterComponent;
use hermes_relayer_components::transaction::traits::event::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::nonce::guard::NonceGuardComponent;
use hermes_relayer_components::transaction::traits::types::{
    FeeTypeComponent, NonceTypeComponent, SignerTypeComponent, TransactionHashTypeComponent,
    TransactionTypeComponent, TxResponseTypeComponent,
};

use crate::impls::transaction::encode_tx::EncodeCosmosTx;
use crate::impls::transaction::estimate_fee::EstimateCosmosTxFee;
use crate::impls::transaction::event::ParseCosmosTxResponseAsEvents;
use crate::impls::transaction::poll_timeout::DefaultPollTimeout;
use crate::impls::transaction::query_nonce::QueryCosmosAccount;
use crate::impls::transaction::query_tx_response::QueryCosmosTxResponse;
use crate::impls::transaction::submit_tx::BroadcastCosmosTx;
use crate::impls::types::chain::ProvideCosmosChainTypes;
use crate::impls::types::transaction::ProvideCosmosTransactionTypes;

pub struct CosmosTxComponents;

delegate_components! {
    CosmosTxComponents {
        [
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            SignerTypeComponent,
            NonceTypeComponent,
            NonceGuardComponent,
            TransactionTypeComponent,
            TransactionHashTypeComponent,
            FeeTypeComponent,
            TxResponseTypeComponent,
        ]:
            ProvideCosmosTransactionTypes,
        PollTimeoutGetterComponent:
            DefaultPollTimeout,
        TxResponseAsEventsParserComponent:
            ParseCosmosTxResponseAsEvents,
        TxResponseQuerierComponent:
            QueryCosmosTxResponse,
        TxEncoderComponent:
            EncodeCosmosTx,
        TxFeeEstimatorComponent:
            EstimateCosmosTxFee,
        TxSubmitterComponent:
            BroadcastCosmosTx,
        NonceQuerierComponent:
            QueryCosmosAccount,
    }
}

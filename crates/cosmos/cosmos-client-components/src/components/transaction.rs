use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::transaction::components::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::traits::encode_tx::TxEncoderComponent;
use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimatorComponent;
use hermes_relayer_components::transaction::traits::nonce::nonce_guard::NonceGuardComponent;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitterComponent;
use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;

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

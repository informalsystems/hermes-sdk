use cgp::prelude::*;
pub use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
pub use hermes_relayer_components::components::default::transaction::DefaultTxComponents;
pub use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
pub use hermes_relayer_components::transaction::traits::encode_tx::TxEncoderComponent;
pub use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimatorComponent;
pub use hermes_relayer_components::transaction::traits::nonce::allocate_nonce::NonceAllocatorComponent;
pub use hermes_relayer_components::transaction::traits::nonce::nonce_guard::NonceGuardComponent;
pub use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
pub use hermes_relayer_components::transaction::traits::parse_events::TxMessageResponseParserComponent;
pub use hermes_relayer_components::transaction::traits::poll_tx_response::TxResponsePollerComponent;
pub use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
pub use hermes_relayer_components::transaction::traits::send_messages_with_signer::MessagesWithSignerSenderComponent;
pub use hermes_relayer_components::transaction::traits::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;
pub use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitterComponent;
pub use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;

pub use crate::impls::queries::eip::dispatch::DispatchQueryEip;
pub use crate::impls::transaction::convert_gas_to_fee::{
    DynamicConvertCosmosGasToFee, StaticConvertCosmosGasToFee,
};
use crate::impls::transaction::encode_tx::EncodeCosmosTx;
use crate::impls::transaction::estimate_fee::EstimateCosmosTxFee;
use crate::impls::transaction::event::ParseCosmosTxResponseAsEvents;
use crate::impls::transaction::poll_timeout::DefaultPollTimeout;
use crate::impls::transaction::query_nonce::QueryCosmosAccount;
use crate::impls::transaction::query_tx_response::QueryCosmosTxResponse;
use crate::impls::transaction::submit_tx::BroadcastCosmosTx;
use crate::impls::types::transaction::ProvideCosmosTransactionTypes;
pub use crate::traits::convert_gas_to_fee::GasToFeeConverterComponent;
pub use crate::traits::eip::eip_query::EipQuerierComponent;

cgp_preset! {
    CosmosChainTxPreset {
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
        [
            MessageSenderComponent,
            MessagesWithSignerSenderComponent,
            MessagesWithSignerAndNonceSenderComponent,
            NonceAllocatorComponent,
            TxResponsePollerComponent,
        ]:
            DefaultTxComponents::Provider,
        PollTimeoutGetterComponent:
            DefaultPollTimeout,
        TxMessageResponseParserComponent:
            ParseCosmosTxResponseAsEvents,
        TxResponseQuerierComponent:
            QueryCosmosTxResponse,
        TxEncoderComponent:
            EncodeCosmosTx,
        TxFeeEstimatorComponent:
            EstimateCosmosTxFee,
        GasToFeeConverterComponent:
            DynamicConvertCosmosGasToFee,
        EipQuerierComponent:
            DispatchQueryEip,
        TxSubmitterComponent:
            BroadcastCosmosTx,
        NonceQuerierComponent:
            QueryCosmosAccount,
    }
}

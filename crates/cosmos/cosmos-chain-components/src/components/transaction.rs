use cgp::prelude::*;
pub use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
pub use hermes_relayer_components::components::default::transaction::DefaultTxComponents;
pub use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
pub use hermes_relayer_components::transaction::traits::encode_tx::TxEncoderComponent;
pub use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimatorComponent;
pub use hermes_relayer_components::transaction::traits::nonce::allocate_nonce::NonceAllocatorComponent;
pub use hermes_relayer_components::transaction::traits::nonce::nonce_guard::NonceGuardComponent;
pub use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
pub use hermes_relayer_components::transaction::traits::parse_events::TxResponseAsEventsParserComponent;
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

use crate::impls::transaction::encode_tx::EncodeCosmosTx;
use crate::impls::transaction::estimate_fee::EstimateCosmosTxFee;
use crate::impls::transaction::event::ParseCosmosTxResponseAsEvents;
use crate::impls::transaction::poll_timeout::DefaultPollTimeout;
use crate::impls::transaction::query_nonce::QueryCosmosAccount;
use crate::impls::transaction::query_tx_response::QueryCosmosTxResponse;
use crate::impls::transaction::submit_tx::BroadcastCosmosTx;
use crate::impls::types::transaction::ProvideCosmosTransactionTypes;

define_components! {
    CosmosTxComponents {
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
            DefaultTxComponents,
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

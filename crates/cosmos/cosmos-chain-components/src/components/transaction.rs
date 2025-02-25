#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
    use hermes_relayer_components::components::default::transaction::DefaultTxComponents;
    use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
    use hermes_relayer_components::transaction::traits::encode_tx::TxEncoderComponent;
    use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimatorComponent;
    use hermes_relayer_components::transaction::traits::nonce::allocate_nonce::NonceAllocatorComponent;
    use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
    use hermes_relayer_components::transaction::traits::parse_events::TxMessageResponseParserComponent;
    use hermes_relayer_components::transaction::traits::poll_tx_response::TxResponsePollerComponent;
    use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
    use hermes_relayer_components::transaction::traits::send_messages_with_signer::MessagesWithSignerSenderComponent;
    use hermes_relayer_components::transaction::traits::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;
    use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitterComponent;
    use hermes_relayer_components::transaction::traits::types::fee::FeeTypeComponent;
    use hermes_relayer_components::transaction::traits::types::nonce::NonceTypeComponent;
    use hermes_relayer_components::transaction::traits::types::signer::SignerTypeComponent;
    use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
    use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
    use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;

    use crate::impls::queries::eip::dispatch::DispatchQueryEip;
    use crate::impls::transaction::convert_gas_to_fee::DynamicConvertCosmosGasToFee;
    use crate::impls::transaction::encode_tx::EncodeCosmosTx;
    use crate::impls::transaction::estimate_fee::EstimateCosmosTxFee;
    use crate::impls::transaction::event::ParseCosmosTxResponseAsEvents;
    use crate::impls::transaction::poll_timeout::FixedPollTimeoutSecs;
    use crate::impls::transaction::query_nonce::QueryCosmosAccount;
    use crate::impls::transaction::query_tx_response::QueryCosmosTxResponse;
    use crate::impls::transaction::submit_tx::BroadcastCosmosTx;
    use crate::impls::types::transaction::ProvideCosmosTransactionTypes;
    use crate::traits::convert_gas_to_fee::GasToFeeConverterComponent;
    use crate::traits::eip::eip_query::EipQuerierComponent;

    cgp_preset! {
        CosmosChainTxPreset {
            [
                SignerTypeComponent,
                NonceTypeComponent,
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
                FixedPollTimeoutSecs<300>,
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
}

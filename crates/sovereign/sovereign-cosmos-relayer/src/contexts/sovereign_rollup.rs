use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_core::HasComponents;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_components::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::types::FeeTypeComponent;
use hermes_relayer_components::transaction::traits::types::NonceTypeComponent;
use hermes_relayer_components::transaction::traits::types::SignerTypeComponent;
use hermes_relayer_components::transaction::traits::types::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::TxResponseTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_client_components::sovereign::components::SovereignClientComponents;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::JsonRpcClientGetter;
use hermes_sovereign_client_components::sovereign::traits::rollup::json_rpc_client::JsonRpcClientTypeComponent;
use hermes_sovereign_client_components::sovereign::traits::rollup::publish_batch::CanPublishTransactionBatch;
use hermes_sovereign_client_components::sovereign::traits::rollup::publish_batch::TransactionBatchPublisherComponent;
use hermes_sovereign_test_components::rollup::components::SovereignRollupTestComponents;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::wallet::WalletTypeComponent;
use jsonrpsee::http_client::HttpClient;

pub struct SovereignRollup {
    pub runtime: HermesRuntime,
    pub rpc_client: HttpClient,
}

pub struct SovereignRollupComponents;

impl HasComponents for SovereignRollup {
    type Components = SovereignRollupComponents;
}

delegate_components! {
    SovereignRollupComponents {
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
        [
            TransactionTypeComponent,
            NonceTypeComponent,
            FeeTypeComponent,
            SignerTypeComponent,
            TransactionHashTypeComponent,
            TxResponseTypeComponent,
            JsonRpcClientTypeComponent,
            TransactionBatchPublisherComponent,
            TxResponseQuerierComponent,
        ]:
            SovereignClientComponents,
        [
            AddressTypeComponent,
            DenomTypeComponent,
            AmountTypeComponent,
            WalletTypeComponent,
            BalanceQuerierComponent,
            EventualAmountAsserterComponent,
            PollAssertDurationGetterComponent,
        ]:
            SovereignRollupTestComponents,
    }
}

impl ProvideRuntime<SovereignRollup> for SovereignRollupComponents {
    fn runtime(rollup: &SovereignRollup) -> &HermesRuntime {
        &rollup.runtime
    }
}

impl JsonRpcClientGetter<SovereignRollup> for SovereignRollupComponents {
    fn json_rpc_client(rollup: &SovereignRollup) -> &HttpClient {
        &rollup.rpc_client
    }
}

pub trait CheckSovereignRollupImpls:
    CanQueryBalance + CanPublishTransactionBatch + CanAssertEventualAmount
{
}

impl CheckSovereignRollupImpls for SovereignRollup {}

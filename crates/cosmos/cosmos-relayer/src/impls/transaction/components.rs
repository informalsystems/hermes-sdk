use cgp_core::{
    delegate_all, delegate_components, DelegateComponent, ErrorRaiserComponent, ErrorTypeComponent,
    HasComponents,
};
use hermes_cosmos_client_components::components::transaction::CosmosTxComponents as BaseCosmosTxComponents;
use hermes_cosmos_client_components::traits::gas_config::GasConfigGetter;
use hermes_cosmos_client_components::traits::grpc_address::GrpcAddressGetter;
use hermes_cosmos_client_components::traits::rpc_client::RpcClientGetter;
use hermes_cosmos_client_components::traits::tx_extension_options::TxExtensionOptionsGetter;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::components::default::transaction::{
    CanUseDefaultTxComponents, DefaultTxComponents, IsDefaultTxComponents,
};
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_components::transaction::components::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::traits::components::tx_encoder::TxEncoderComponent;
use hermes_relayer_components::transaction::traits::components::tx_fee_estimater::TxFeeEstimatorComponent;
use hermes_relayer_components::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::components::tx_submitter::TxSubmitterComponent;
use hermes_relayer_components::transaction::traits::event::TxResponseAsEventsParserComponent;
use hermes_relayer_components::transaction::traits::nonce::guard::NonceGuardComponent;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
use hermes_relayer_components::transaction::traits::types::{
    fee::FeeTypeComponent, nonce::NonceTypeComponent, signer::SignerTypeComponent,
    transaction::TransactionTypeComponent, tx_hash::TransactionHashTypeComponent,
    tx_response::TxResponseTypeComponent,
};
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use http::Uri;
use ibc_proto::google::protobuf::Any;
use ibc_relayer::chain::cosmos::types::gas::GasConfig;
use tendermint_rpc::{HttpClient, Url};

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::error::HandleCosmosError;

pub struct CosmosTxComponents;

impl HasComponents for CosmosTxContext {
    type Components = CosmosTxComponents;
}

delegate_all!(
    IsDefaultTxComponents,
    DefaultTxComponents,
    CosmosTxComponents,
);

impl CanUseDefaultTxComponents for CosmosTxContext {}

delegate_components! {
    CosmosTxComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
        [
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            SignerTypeComponent,
            NonceTypeComponent,
            NonceGuardComponent,
            TransactionTypeComponent,
            TransactionHashTypeComponent,
            FeeTypeComponent,
            TxResponseTypeComponent,
            PollTimeoutGetterComponent,
            TxResponseAsEventsParserComponent,
            TxResponseQuerierComponent,
            TxEncoderComponent,
            TxFeeEstimatorComponent,
            TxSubmitterComponent,
            NonceQuerierComponent,
        ]:
            BaseCosmosTxComponents,
    }
}

impl RpcClientGetter<CosmosTxContext> for CosmosTxComponents {
    fn rpc_client(chain: &CosmosTxContext) -> &HttpClient {
        &chain.rpc_client
    }

    fn rpc_address(chain: &CosmosTxContext) -> &Url {
        &chain.tx_config.rpc_address
    }
}

impl GrpcAddressGetter<CosmosTxContext> for CosmosTxComponents {
    fn grpc_address(chain: &CosmosTxContext) -> &Uri {
        &chain.tx_config.grpc_address
    }
}

impl TxExtensionOptionsGetter<CosmosTxContext> for CosmosTxComponents {
    fn tx_extension_options(chain: &CosmosTxContext) -> &Vec<Any> {
        &chain.tx_config.extension_options
    }
}

impl GasConfigGetter<CosmosTxContext> for CosmosTxComponents {
    fn gas_config(chain: &CosmosTxContext) -> &GasConfig {
        &chain.tx_config.gas_config
    }
}

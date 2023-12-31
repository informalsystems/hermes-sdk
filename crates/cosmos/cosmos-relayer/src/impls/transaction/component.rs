use cgp_core::{
    delegate_all, delegate_components, DelegateComponent, ErrorRaiserComponent, ErrorTypeComponent,
    HasComponents,
};
use hermes_cosmos_client_components::components::types::chain::ProvideCosmosChainTypes;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use hermes_relayer_components::components::default::transaction::{
    CanUseDefaultTxComponents, DefaultTxComponents, IsDefaultTxComponents,
};
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

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
            ChainIdTypeProviderComponent,
            MessageTypeProviderComponent,
            EventTypeProviderComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
    }
}

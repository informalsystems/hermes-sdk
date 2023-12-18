use cgp_core::{delegate_components, ErrorRaiserComponent, ErrorTypeComponent, HasComponents};
use cosmos_client_components::components::types::chain::ProvideCosmosChainTypes;
use ibc_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use ibc_relayer_components_extra::components::extra::transaction::ExtraTxComponents;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use ibc_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::error::HandleCosmosError;

pub struct CosmosTxComponents;

impl HasComponents for CosmosTxContext {
    type Components = ExtraTxComponents<CosmosTxComponents>;
}

delegate_components!(
    CosmosTxComponents;
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
);

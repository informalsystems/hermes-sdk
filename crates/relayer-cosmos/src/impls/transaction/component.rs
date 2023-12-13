use cgp_core::{delegate_components, HasComponents};
use cosmos_client_components::components::types::chain::ProvideCosmosChainTypes;
use ibc_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components_extra::components::extra::transaction::ExtraTxComponents;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;

use crate::contexts::transaction::CosmosTxContext;

pub struct CosmosTxComponents;

impl HasComponents for CosmosTxContext {
    type Components = ExtraTxComponents<CosmosTxComponents>;
}

delegate_components!(
    CosmosTxComponents;
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

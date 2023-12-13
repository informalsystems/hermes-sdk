use cgp_core::prelude::*;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;

pub struct MockCosmosChainComponents;

delegate_components!(
    MockCosmosChainComponents;
    [
        LoggerTypeComponent,
        LoggerFieldComponent,
    ]:
        ProvideTracingLogger,
);

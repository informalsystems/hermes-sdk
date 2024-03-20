use cgp_core::prelude::*;
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;

pub struct MockCosmosChainComponents;

delegate_components! {
    MockCosmosChainComponents {
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
    }
}

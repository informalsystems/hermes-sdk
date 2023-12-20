use cgp_core::delegate_components;
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;

pub struct MockChainComponents;

delegate_components! {
    MockChainComponents {
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
    }
}

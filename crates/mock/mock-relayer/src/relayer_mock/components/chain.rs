use cgp_core::delegate_components;
use hermes_relayer_components::log::contexts::no_logger::ProvideNoLogger;
use hermes_relayer_components::log::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;

pub struct MockChainComponents;

delegate_components! {
    MockChainComponents {
        [
            hermes_relayer_components::logger::traits::has_logger::LoggerTypeComponent,
            hermes_relayer_components::logger::traits::has_logger::LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
    }
}

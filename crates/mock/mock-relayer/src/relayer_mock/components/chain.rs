use cgp_core::delegate_components;
use hermes_relayer_components::log::contexts::no_logger::ProvideNoLogger;
use hermes_relayer_components::log::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};

pub struct MockChainComponents;

delegate_components! {
    MockChainComponents {
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
    }
}

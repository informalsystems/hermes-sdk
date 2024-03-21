use cgp_core::{delegate_all, delegate_components, DelegateComponent};
use hermes_relayer_components::components::default::relay::{
    DefaultRelayComponents, IsDefaultRelayComponent,
};
use hermes_relayer_components::log::contexts::no_logger::ProvideNoLogger;
use hermes_relayer_components::log::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::relay::impls::packet_filters::allow_all::AllowAll;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilterComponent;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;

pub struct MockCosmosRelayComponents;

delegate_all!(
    IsDefaultRelayComponent,
    DefaultRelayComponents,
    MockCosmosRelayComponents,
);

delegate_components! {
    MockCosmosRelayComponents {
        PacketFilterComponent: AllowAll,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
    }
}

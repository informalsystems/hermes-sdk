use cgp_core::prelude::*;
use hermes_logging_components::contexts::no_logger::ProvideNoLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::relay::impls::packet_filters::allow_all::AllowAll;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilterComponent;
use hermes_relayer_components::with_default_relay_components;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};

pub struct MockCosmosRelayComponents;

with_default_relay_components! {
    delegate_components! {
        MockCosmosRelayComponents {
            @DefaultRelayComponents : DefaultRelayComponents,
        }
    }
}

delegate_components! {
    MockCosmosRelayComponents {
        PacketFilterComponent: AllowAll,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
    }
}

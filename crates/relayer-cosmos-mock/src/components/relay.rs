use ibc_relayer_components::components::default::relay::{
    DefaultRelayComponents, IsDefaultRelayComponent,
};
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};

use ibc_relayer_components::relay::components::packet_filters::allow_all::AllowAll;

use ibc_relayer_components::relay::traits::components::packet_filter::PacketFilterComponent;

use cgp_core::{delegate_all, delegate_components, DelegateComponent};
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use ibc_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

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
            ProvideTokioRuntimeType,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
    }
}

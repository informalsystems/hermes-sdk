use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use hermes_relayer_components::components::default::birelay::{
    DefaultBiRelayComponents, IsDefaultBiRelayComponent,
};
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

use crate::contexts::birelay::CosmosBiRelay;
use crate::impls::error::HandleCosmosError;

pub struct CosmosBiRelayComponents;

impl<ChainA, ChainB> HasComponents for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: Async,
    ChainB: Async,
{
    type Components = CosmosBiRelayComponents;
}

delegate_all!(
    IsDefaultBiRelayComponent,
    DefaultBiRelayComponents,
    CosmosBiRelayComponents,
);

delegate_components! {
    CosmosBiRelayComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
    }
}

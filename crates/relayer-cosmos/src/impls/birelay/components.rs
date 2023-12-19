use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use ibc_relayer_components::components::default::birelay::DefaultBiRelayComponents;
use ibc_relayer_components::components::default::birelay::IsDefaultBiRelayComponent;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use ibc_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

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

impl<Component> DelegateComponent<Component> for CosmosBiRelayComponents
where
    Self: IsDefaultBiRelayComponent<Component>,
{
    type Delegate = DefaultBiRelayComponents;
}

delegate_components!(
    CosmosBiRelayComponents;
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
);

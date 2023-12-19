use cgp_core::delegate_all;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use ibc_relayer_components_extra::components::extra::closures::relay::auto_relayer::CanUseExtraAutoRelayer;
use ibc_relayer_components_extra::components::extra::relay::ExtraRelayComponents;
use ibc_relayer_components_extra::components::extra::relay::IsExtraRelayComponent;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use ibc_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

use crate::contexts::relay::CosmosRelay;
use crate::impls::error::HandleCosmosError;

pub struct CosmosRelayComponents;

delegate_components!(
    CosmosRelayComponents;
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

delegate_all!(
    IsExtraRelayComponent,
    ExtraRelayComponents,
    CosmosRelayComponents,
);

impl<SrcChain, DstChain> HasComponents for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: Async,
    DstChain: Async,
{
    type Components = CosmosRelayComponents;
}

impl<SrcChain, DstChain> CanUseExtraAutoRelayer for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
}

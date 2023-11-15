use cgp_core::prelude::*;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components_extra::components::extra::closures::relay::auto_relayer::CanUseExtraAutoRelayer;
use ibc_relayer_components_extra::components::extra::relay::ExtraRelayComponents;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;

use crate::contexts::relay::CosmosRelay;

pub struct CosmosRelayComponents;

delegate_components!(
    CosmosRelayComponents;
    [
        LoggerTypeComponent,
        LoggerFieldComponent,
    ]:
        ProvideTracingLogger,
);

impl<SrcChain, DstChain> HasComponents for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: Async,
    DstChain: Async,
{
    type Components = ExtraRelayComponents<CosmosRelayComponents>;
}

impl<SrcChain, DstChain> CanUseExtraAutoRelayer for CosmosRelay<SrcChain, DstChain>
where
    SrcChain: ChainHandle,
    DstChain: ChainHandle,
{
}

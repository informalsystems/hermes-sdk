use cgp_core::prelude::*;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components_extra::components::extra::birelay::ExtraBiRelayComponents;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;

use crate::contexts::birelay::CosmosBiRelay;

pub struct CosmosBiRelayComponents;

impl<ChainA, ChainB> HasComponents for CosmosBiRelay<ChainA, ChainB>
where
    ChainA: Async,
    ChainB: Async,
{
    type Components = ExtraBiRelayComponents<CosmosBiRelayComponents>;
}

delegate_components!(
    CosmosBiRelayComponents;
    [
        LoggerTypeComponent,
        LoggerFieldComponent,
    ]:
        ProvideTracingLogger,
);

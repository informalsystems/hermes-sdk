use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_core::RunnerComponent;
use ibc_relayer_components::components::default::birelay::DefaultBiRelayComponents;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components::runtime::traits::runtime::RuntimeComponent;
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;

pub struct ExtraBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    ExtraBiRelayComponents<BaseComponents>;
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
        RuntimeTypeComponent,
        RuntimeComponent,
        RunnerComponent,
        LoggerTypeComponent,
        LoggerFieldComponent,
    ]:
        DefaultBiRelayComponents<BaseComponents>,
);

use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_core::RunnerComponent;

use crate::logger::traits::has_logger::{LoggerFieldComponent, LoggerTypeComponent};
use crate::relay::components::auto_relayers::both_ways::RelayBothWays;
use crate::runtime::traits::runtime::RuntimeComponent;
use crate::runtime::traits::runtime::RuntimeTypeComponent;

pub struct DefaultBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    DefaultBiRelayComponents<BaseComponents>;
    RunnerComponent: RelayBothWays,
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
        RuntimeTypeComponent,
        RuntimeComponent,
        LoggerTypeComponent,
        LoggerFieldComponent,
    ]:
        BaseComponents,
);

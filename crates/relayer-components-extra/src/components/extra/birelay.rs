use core::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::RunnerComponent;
use ibc_relayer_components::components::default::birelay::DefaultBiRelayComponents;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};

pub struct ExtraBiRelayComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    ExtraBiRelayComponents<BaseComponents>;
    [
        RunnerComponent,
        LoggerTypeComponent,
        LoggerFieldComponent,
    ]:
        DefaultBiRelayComponents<BaseComponents>,
);

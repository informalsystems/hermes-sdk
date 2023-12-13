use cgp_core::prelude::*;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components_extra::components::extra::build::ExtraBuildComponents;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;

use crate::contexts::builder::CosmosBuilder;

pub struct CosmosBuildComponents;

impl HasComponents for CosmosBuilder {
    type Components = ExtraBuildComponents<CosmosBuildComponents>;
}

delegate_components!(
    CosmosBuildComponents;
    [
        LoggerTypeComponent,
        LoggerFieldComponent,
    ]:
        ProvideTracingLogger,
);

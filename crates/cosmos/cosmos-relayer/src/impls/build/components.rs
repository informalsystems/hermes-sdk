use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_components_extra::components::extra::build::{
    CanUseExtraBuildComponents, ExtraBuildComponents, IsExtraBuildComponent,
};
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

use crate::contexts::builder::CosmosBuilder;
use crate::impls::error::HandleCosmosError;

pub struct CosmosBuildComponents;

pub struct CosmosBaseBuildComponents;

impl HasComponents for CosmosBuilder {
    type Components = CosmosBuildComponents;
}

impl HasComponents for CosmosBuildComponents {
    type Components = CosmosBaseBuildComponents;
}

delegate_all!(
    IsExtraBuildComponent,
    ExtraBuildComponents<CosmosBaseBuildComponents>,
    CosmosBuildComponents,
);

impl CanUseExtraBuildComponents for CosmosBuilder {}

delegate_components! {
    CosmosBuildComponents {
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

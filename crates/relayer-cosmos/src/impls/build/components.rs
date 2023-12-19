use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use ibc_relayer_components_extra::components::extra::build::ExtraBuildComponents;
use ibc_relayer_components_extra::components::extra::build::IsExtraBuildComponent;
use ibc_relayer_components_extra::components::extra::closures::build::CanUseExtraBuilderComponents;
use ibc_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use ibc_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

use crate::contexts::builder::CosmosBuilder;
use crate::impls::build::chain::BuildCosmosChain;
use crate::impls::error::HandleCosmosError;

pub struct CosmosBuildComponents;

impl HasComponents for CosmosBuilder {
    type Components = CosmosBuildComponents;
}

impl<Component> DelegateComponent<Component> for CosmosBuildComponents
where
    Self: IsExtraBuildComponent<Component>,
{
    type Delegate = ExtraBuildComponents<BuildCosmosChain>;
}

impl CanUseExtraBuilderComponents<BuildCosmosChain> for CosmosBuilder {}

delegate_components!(
    CosmosBuildComponents;
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

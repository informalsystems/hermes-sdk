use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_relayer_components_extra::components::extra::build::*;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::{
    GetRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};

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

with_extra_build_components! {
    delegate_components! {
        CosmosBuildComponents {
            @ExtraBuildComponents: ExtraBuildComponents<CosmosBaseBuildComponents>
        }
    }
}

impl CanUseExtraBuildComponents for CosmosBuilder {}

delegate_components! {
    CosmosBuildComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        RuntimeGetterComponent:
            GetRuntimeField<symbol!("runtime")>,
    }
}

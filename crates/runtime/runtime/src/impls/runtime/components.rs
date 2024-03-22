use cgp_core::delegate_all;
use cgp_core::prelude::*;
use hermes_tokio_runtime_components::components::parallel::{
    IsTokioParallelRuntimeComponent, TokioParallelRuntimeComponents,
};

use crate::types::runtime::HermesRuntime;

pub struct HermesRuntimeComponents;

impl HasComponents for HermesRuntime {
    type Components = HermesRuntimeComponents;
}

delegate_all!(
    IsTokioParallelRuntimeComponent,
    TokioParallelRuntimeComponents,
    HermesRuntimeComponents,
);

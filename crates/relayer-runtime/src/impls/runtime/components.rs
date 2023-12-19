use cgp_core::delegate_all;
use cgp_core::prelude::*;
use tokio_runtime_components::components::runtime::{
    IsTokioRuntimeComponent, TokioRuntimeComponents,
};

use crate::types::runtime::TokioRuntimeContext;

pub struct RelayerRuntimeComponents;

impl HasComponents for TokioRuntimeContext {
    type Components = RelayerRuntimeComponents;
}

delegate_all!(
    IsTokioRuntimeComponent,
    TokioRuntimeComponents,
    RelayerRuntimeComponents,
);

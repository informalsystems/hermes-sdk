use cgp_core::prelude::*;
use tokio_runtime_components::components::runtime::{
    IsTokioRuntimeComponent, TokioRuntimeComponents,
};

use crate::types::runtime::TokioRuntimeContext;

pub struct RelayerRuntimeComponents;

impl HasComponents for TokioRuntimeContext {
    type Components = RelayerRuntimeComponents;
}

impl<Component> DelegateComponent<Component> for RelayerRuntimeComponents
where
    RelayerRuntimeComponents: IsTokioRuntimeComponent<Component>,
{
    type Delegate = TokioRuntimeComponents;
}

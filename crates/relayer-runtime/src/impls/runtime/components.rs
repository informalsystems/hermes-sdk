use cgp_core::prelude::*;
use tokio_runtime_components::components::runtime::{
    IsTokioRuntimeComponent, TokioRuntimeComponents,
};

use crate::types::runtime::TokioRuntimeContext;

pub struct RelayerRuntimeComponents;

impl HasComponents for TokioRuntimeContext {
    type Components = RelayerRuntimeComponents;
}

pub trait ToBeDelegated {}

impl<Component> ToBeDelegated for Component where
    RelayerRuntimeComponents: IsTokioRuntimeComponent<Component>
{
}

impl<Component> DelegateComponent<Component> for RelayerRuntimeComponents
where
    Component: ToBeDelegated,
{
    type Delegate = TokioRuntimeComponents;
}

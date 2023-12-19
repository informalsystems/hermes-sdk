use cgp_core::prelude::*;
use ibc_relayer_components::components::default::relay::{
    DefaultRelayComponents, IsDefaultRelayComponent,
};
use ibc_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use ibc_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;

use crate::context::relay::SolomachineRelay;

pub struct SolomachineRelayComponents;

impl<Component> DelegateComponent<Component> for SolomachineRelayComponents
where
    Self: IsDefaultRelayComponent<Component>,
{
    type Delegate = DefaultRelayComponents;
}

impl<Chain> HasComponents for SolomachineRelay<Chain>
where
    Chain: Async,
{
    type Components = SolomachineRelayComponents;
}

delegate_components!(
    SolomachineRelayComponents;
    RuntimeTypeComponent:
        ProvideTokioRuntimeType,
);

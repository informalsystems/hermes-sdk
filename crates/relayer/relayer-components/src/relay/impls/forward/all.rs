use cgp_core::prelude::*;

use crate::relay::impls::forward::create_client::ForwardCreateClient;
use crate::relay::impls::forward::types::ForwardRelayTypes;
use crate::relay::traits::chains::RelayChainsComponent;
use crate::relay::traits::components::client_creator::ClientCreatorComponent;

pub struct ForwardToInnerRelay;

delegate_components! {
    #[mark_component(IsForwardToInnerRelayComponent)]
    #[mark_delegate(DelegatesToInnerRelay)]
    ForwardToInnerRelay {
        RelayChainsComponent: ForwardRelayTypes,
        ClientCreatorComponent: ForwardCreateClient,
    }
}

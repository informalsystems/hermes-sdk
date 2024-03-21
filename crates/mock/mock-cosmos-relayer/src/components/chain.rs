use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;

pub struct MockCosmosChainComponents;

delegate_components! {
    MockCosmosChainComponents {
        RuntimeTypeComponent:
            ProvideHermesRuntime,
    }
}

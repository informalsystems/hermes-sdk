use cgp_core::prelude::*;
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;

pub struct MockCosmosChainComponents;

delegate_components! {
    MockCosmosChainComponents {
        RuntimeTypeComponent:
            ProvideHermesRuntime,
    }
}

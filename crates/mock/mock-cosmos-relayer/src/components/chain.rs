use cgp_core::prelude::*;
use hermes_relayer_components::chain::impls::types::ack::ProvideBytesAcknowlegement;
use hermes_relayer_components::chain::traits::types::ack::AcknowledgementTypeComponent;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;

pub struct MockCosmosChainComponents;

delegate_components! {
    MockCosmosChainComponents {
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        AcknowledgementTypeComponent:
            ProvideBytesAcknowlegement,
    }
}

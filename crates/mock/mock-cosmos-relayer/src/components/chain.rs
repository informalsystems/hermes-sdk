use cgp::prelude::*;
use hermes_relayer_components::chain::impls::types::ack::ProvideBytesAcknowlegement;
use hermes_relayer_components::chain::traits::types::packets::ack::AcknowledgementTypeComponent;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};

pub struct MockCosmosChainComponents;

delegate_components! {
    MockCosmosChainComponents {
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        AcknowledgementTypeComponent:
            ProvideBytesAcknowlegement,
    }
}

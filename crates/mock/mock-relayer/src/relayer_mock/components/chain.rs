use cgp::prelude::*;
use hermes_logging_components::impls::IgnoreLog;
use hermes_logging_components::traits::LoggerComponent;
use hermes_relayer_components::chain::traits::{
    AckCommitmentHashTypeProviderComponent, AcknowledgementTypeProviderComponent,
};

pub use crate::relayer_mock::contexts::chain::MockChainComponents;

delegate_components! {
    MockChainComponents {
        LoggerComponent: IgnoreLog,
        [
            AcknowledgementTypeProviderComponent,
            AckCommitmentHashTypeProviderComponent,
        ]:
            UseType<Vec<u8>>,
    }
}

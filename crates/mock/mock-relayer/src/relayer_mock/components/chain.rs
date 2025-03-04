use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_logging_components::contexts::no_logger::ProvideNoLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeProviderComponent,
};
use hermes_relayer_components::chain::impls::types::ack::ProvideBytesAcknowlegement;
use hermes_relayer_components::chain::traits::types::packets::ack::{
    AckCommitmentHashTypeComponent, AcknowledgementTypeComponent,
};

pub use crate::relayer_mock::contexts::chain::MockChainComponents;

delegate_components! {
    MockChainComponents {
        [
            LoggerTypeProviderComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
        AcknowledgementTypeComponent:
            ProvideBytesAcknowlegement,
        AckCommitmentHashTypeComponent:
            WithType<Vec<u8>>,
    }
}

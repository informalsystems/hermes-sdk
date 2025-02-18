use cgp::prelude::*;
use hermes_logging_components::contexts::no_logger::ProvideNoLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::impls::types::ack::ProvideBytesAcknowlegement;
use hermes_relayer_components::chain::traits::types::packets::ack::AcknowledgementTypeComponent;

pub use crate::relayer_mock::contexts::chain::MockChainComponents;

delegate_components! {
    MockChainComponents {
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
        AcknowledgementTypeComponent:
            ProvideBytesAcknowlegement,
    }
}

use cgp_core::Async;
use hermes_relayer_components::log::traits::logger::Logger;
use tracing::info;

use crate::contexts::logger::TracingLogger;

impl<Logging> Logger<Logging, ()> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &()) {
        info!("{message}");
    }
}

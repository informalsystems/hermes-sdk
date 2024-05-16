use core::time::Duration;

use ibc::core::connection::types::version::Version;

pub struct SovereignInitConnectionOptions {
    pub delay_period: Duration,
    pub connection_version: Version,
}

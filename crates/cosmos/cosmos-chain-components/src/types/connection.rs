use core::time::Duration;

use ibc::core::connection::types::version::Version;

#[derive(Clone, Debug)]
pub struct CosmosInitConnectionOptions {
    pub delay_period: Duration,
    pub connection_version: Version,
}

impl Default for CosmosInitConnectionOptions {
    fn default() -> Self {
        Self {
            delay_period: Default::default(),
            // Can unwrap safely since compatibles() returns a vector containing 1 Version
            connection_version: Version::compatibles().first().unwrap().clone(),
        }
    }
}

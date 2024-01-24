use hermes_celestia_test_components::types::bridge_config::CelestiaBridgeConfig;
use tokio::process::Child;

pub struct CelestiaBridgeDriver {
    pub bridge_process: Child,
    pub bridge_config: CelestiaBridgeConfig,
}

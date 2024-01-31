use cgp_core::{Async, HasComponents};
use hermes_celestia_test_components::bridge_driver::traits::bridge_auth_token::ProvideBridgeAuthTokenType;
use hermes_celestia_test_components::types::bridge_config::CelestiaBridgeConfig;
use tokio::process::Child;

pub struct CelestiaBridgeDriver {
    pub bridge_process: Child,
    pub bridge_config: CelestiaBridgeConfig,
}

pub struct CelestiaBridgeDriverComponents;

impl HasComponents for CelestiaBridgeDriver {
    type Components = CelestiaBridgeDriverComponents;
}

impl<BridgeDriver> ProvideBridgeAuthTokenType<BridgeDriver> for CelestiaBridgeDriverComponents
where
    BridgeDriver: Async,
{
    type BridgeAuthToken = String;
}

use cgp::prelude::*;
use hermes_celestia_test_components::bridge_driver::traits::bridge_auth_token::{
    BridgeAuthTokenGetter, BridgeAuthTokenGetterComponent, BridgeAuthTokenTypeComponent,
    ProvideBridgeAuthTokenType,
};
use hermes_celestia_test_components::bridge_driver::traits::bridge_rpc_port::{
    BridgeRpcPortGetter, BridgeRpcPortGetterComponent,
};
use hermes_celestia_test_components::types::bridge_config::CelestiaBridgeConfig;
use tokio::process::Child;

pub struct CelestiaBridgeDriver {
    pub bridge_process: Child,
    pub bridge_config: CelestiaBridgeConfig,
    pub bridge_auth_token: String,
}

pub struct CelestiaBridgeDriverComponents;

impl HasComponents for CelestiaBridgeDriver {
    type Components = CelestiaBridgeDriverComponents;
}

#[cgp_provider(BridgeAuthTokenTypeComponent)]
impl<BridgeDriver> ProvideBridgeAuthTokenType<BridgeDriver> for CelestiaBridgeDriverComponents
where
    BridgeDriver: Async,
{
    type BridgeAuthToken = String;
}

#[cgp_provider(BridgeAuthTokenGetterComponent)]
impl BridgeAuthTokenGetter<CelestiaBridgeDriver> for CelestiaBridgeDriverComponents {
    fn bridge_auth_token(bridge_driver: &CelestiaBridgeDriver) -> &String {
        &bridge_driver.bridge_auth_token
    }
}

#[cgp_provider(BridgeRpcPortGetterComponent)]
impl BridgeRpcPortGetter<CelestiaBridgeDriver> for CelestiaBridgeDriverComponents {
    fn bridge_rpc_port(bridge_driver: &CelestiaBridgeDriver) -> u16 {
        bridge_driver.bridge_config.bridge_rpc_port
    }
}

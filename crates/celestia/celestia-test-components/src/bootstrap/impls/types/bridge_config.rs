use cgp_core::Async;

use crate::bootstrap::traits::types::bridge_config::ProvideBridgeConfigType;
use crate::types::bridge_config::CelestiaBridgeConfig;

pub struct ProvideCelestiaBridgeConfig;

impl<Bootstrap> ProvideBridgeConfigType<Bootstrap> for ProvideCelestiaBridgeConfig
where
    Bootstrap: Async,
{
    type BridgeConfig = CelestiaBridgeConfig;
}

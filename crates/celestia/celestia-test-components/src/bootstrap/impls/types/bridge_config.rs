use hermes_prelude::*;

use crate::bootstrap::traits::types::bridge_config::{
    BridgeConfigTypeComponent, ProvideBridgeConfigType,
};
use crate::types::bridge_config::CelestiaBridgeConfig;

pub struct ProvideCelestiaBridgeConfig;

#[cgp_provider(BridgeConfigTypeComponent)]
impl<Bootstrap> ProvideBridgeConfigType<Bootstrap> for ProvideCelestiaBridgeConfig
where
    Bootstrap: Async,
{
    type BridgeConfig = CelestiaBridgeConfig;
}

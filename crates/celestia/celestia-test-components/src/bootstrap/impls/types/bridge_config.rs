use cgp_core::Async;

use crate::bootstrap::traits::types::bridge_config::ProvideBridgeConfigType;

pub struct ProvideTomlBridgeConfig;

impl<Bootstrap> ProvideBridgeConfigType<Bootstrap> for ProvideTomlBridgeConfig
where
    Bootstrap: Async,
{
    type BridgeConfig = toml::Value;
}

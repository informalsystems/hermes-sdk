use async_trait::async_trait;
use cgp_macros::derive_component;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;
use crate::std_prelude::*;

#[derive_component(AutoRelayerComponent, AutoRelayer<Relay>)]
#[async_trait]
pub trait CanAutoRelay<Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn auto_relay(&self, target: Target) -> Result<(), Self::Error>;
}

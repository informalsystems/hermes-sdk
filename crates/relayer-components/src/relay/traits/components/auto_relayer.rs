use async_trait::async_trait;
use cgp_core::traits::Async;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;
use crate::std_prelude::*;

/// Similar to the `CanAutoRelay` trait, the main differences are that this
/// trait only relays to a specific target, i.e., in one direction, as well
/// as the fact that it is specific to the `Relay` context.
#[async_trait]
pub trait AutoRelayerWithTarget<Relay, Target>: Async
where
    Relay: HasRelayChains,
    Target: ChainTarget<Relay>,
{
    /// Starts the auto-relaying process of relaying to the given `Relay` context's
    /// target.
    async fn auto_relay_with_target(relay: &Relay) -> Result<(), Relay::Error>;
}

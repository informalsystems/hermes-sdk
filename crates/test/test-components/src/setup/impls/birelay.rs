use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::error::{CanRaiseAsyncError, ErrorOf};
use cgp::core::Async;
use hermes_prelude::*;
use hermes_relayer_components::build::traits::builders::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use hermes_relayer_components::chain::traits::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::{
    HasBoundedRelayTypeAt, HasRelayTypeAt, RelayAt,
};

use crate::setup::traits::{BiRelaySetup, BiRelaySetupComponent, CanSetupRelays, HasBuilderAt};

#[cgp_new_provider(BiRelaySetupComponent)]
impl<Setup, A: Async, B: Async> BiRelaySetup<Setup, A, B> for SetupBiRelayWithBuilder
where
    Setup: HasBiRelayTypeAt<A, B>
        + HasBuilderAt<A, B>
        + CanSetupRelays<A, B>
        + HasBoundedRelayTypeAt<A, B>
        + HasBoundedRelayTypeAt<B, A>
        + CanRaiseAsyncError<ErrorOf<Setup::Builder>>,
    ChainAt<Setup, A>: HasIbcChainTypes<ChainAt<Setup, B>> + Clone,
    ChainAt<Setup, B>: HasIbcChainTypes<ChainAt<Setup, A>> + Clone,
    Setup::Builder: CanBuildBiRelayFromRelays<A, B, BiRelay = Setup::BiRelay>
        + HasRelayTypeAt<A, B, Relay = RelayAt<Setup, A, B>>
        + HasRelayTypeAt<B, A, Relay = RelayAt<Setup, B, A>>,
{
    async fn setup_birelay(
        setup: &Setup,
        _index: PhantomData<(A, B)>,
        chain_a: &ChainAt<Setup, A>,
        chain_b: &ChainAt<Setup, B>,
        client_id_a: &ClientIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>,
        client_id_b: &ClientIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>,
        refresh_rate_a: Option<Duration>,
        refresh_rate_b: Option<Duration>,
    ) -> Result<BiRelayAt<Setup, A, B>, Setup::Error> {
        let (relay_a_to_b, relay_b_to_a) = setup
            .setup_relays(
                PhantomData,
                chain_a,
                chain_b,
                client_id_a,
                client_id_b,
                refresh_rate_a,
                refresh_rate_b,
            )
            .await?;

        let birelay = setup
            .builder()
            .build_birelay_from_relays(relay_a_to_b, relay_b_to_a)
            .await
            .map_err(Setup::raise_error)?;

        Ok(birelay)
    }
}

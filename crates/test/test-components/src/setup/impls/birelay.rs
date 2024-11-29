use cgp::core::error::{CanRaiseError, ErrorOf};
use hermes_relayer_components::build::traits::builders::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::{
    HasBoundedRelayTypeAt, HasRelayTypeAt, RelayAt,
};
use hermes_relayer_components::multi::types::index::Twindex;

use crate::setup::traits::birelay::BiRelaySetup;
use crate::setup::traits::builder_at::HasBuilderAt;
use crate::setup::traits::relay::CanSetupRelays;

pub struct SetupBiRelayWithBuilder;

impl<Setup, const A: usize, const B: usize> BiRelaySetup<Setup, A, B> for SetupBiRelayWithBuilder
where
    Setup: HasBiRelayTypeAt<A, B>
        + HasBuilderAt<A, B>
        + CanSetupRelays<A, B>
        + HasBoundedRelayTypeAt<A, B>
        + HasBoundedRelayTypeAt<B, A>
        + CanRaiseError<ErrorOf<Setup::Builder>>,
    ChainAt<Setup, A>: HasIbcChainTypes<ChainAt<Setup, B>> + Clone,
    ChainAt<Setup, B>: HasIbcChainTypes<ChainAt<Setup, A>> + Clone,
    Setup::Builder: CanBuildBiRelayFromRelays<0, 1>
        + HasRelayTypeAt<0, 1, Relay = RelayAt<Setup, A, B>>
        + HasRelayTypeAt<1, 0, Relay = RelayAt<Setup, B, A>>,
{
    async fn setup_birelay(
        setup: &Setup,
        _index: Twindex<A, B>,
        chain_a: &ChainAt<Setup, A>,
        chain_b: &ChainAt<Setup, B>,
        client_id_a: &ClientIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>,
        client_id_b: &ClientIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>,
    ) -> Result<BiRelayAt<Setup, A, B>, Setup::Error> {
        let (relay_a_to_b, relay_b_to_a) = setup
            .setup_relays(Twindex::<A, B>, chain_a, chain_b, client_id_a, client_id_b)
            .await?;

        let birelay = setup
            .builder()
            .build_birelay_from_relays(relay_a_to_b, relay_b_to_a)
            .await
            .map_err(Setup::raise_error)?;

        Ok(birelay)
    }
}

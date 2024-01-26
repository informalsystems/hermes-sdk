use cgp_core::CanRaiseError;
use hermes_relayer_components::build::traits::components::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientId;

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::setup::traits::birelay::BiRelaySetup;
use crate::setup::traits::builder_at::HasBuilderAt;
use crate::setup::traits::relay::CanSetupRelays;
use crate::types::error::ErrorOf;
use crate::types::index::Twindex;

pub struct SetupBiRelayWithBuilder;

impl<Setup, const A: usize, const B: usize> BiRelaySetup<Setup, A, B> for SetupBiRelayWithBuilder
where
    Setup: HasBiRelayTypeAt<A, B>
        + HasBuilderAt<A, B>
        + CanSetupRelays<A, B>
        + CanRaiseError<ErrorOf<Setup::Builder>>,
    ChainTypeAt<Setup, A>: HasIbcChainTypes<ChainTypeAt<Setup, B>> + Clone,
    ChainTypeAt<Setup, B>: HasIbcChainTypes<ChainTypeAt<Setup, A>> + Clone,
    Setup::Builder: CanBuildBiRelayFromRelays,
{
    async fn setup_birelay(
        setup: &Setup,
        _index: Twindex<A, B>,
        chain_a: &ChainTypeAt<Setup, A>,
        chain_b: &ChainTypeAt<Setup, B>,
        client_id_a: &ClientId<ChainTypeAt<Setup, A>, ChainTypeAt<Setup, B>>,
        client_id_b: &ClientId<ChainTypeAt<Setup, B>, ChainTypeAt<Setup, A>>,
    ) -> Result<BiRelayTypeAt<Setup, A, B>, Setup::Error> {
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

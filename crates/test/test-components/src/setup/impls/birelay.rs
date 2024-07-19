use cgp_core::error::{CanRaiseError, ErrorOf};
use hermes_relayer_components::build::traits::builders::birelay_from_relay_builder::CanBuildBiRelayFromRelays;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, RelayTypeAt};
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
        + CanRaiseError<ErrorOf<Setup::Builder>>,
    ChainTypeAt<Setup, A>: HasIbcChainTypes<ChainTypeAt<Setup, B>> + Clone,
    ChainTypeAt<Setup, B>: HasIbcChainTypes<ChainTypeAt<Setup, A>> + Clone,
    Setup::Builder: CanBuildBiRelayFromRelays<0, 1>
        + HasRelayTypeAt<0, 1, Relay = RelayTypeAt<Setup, A, B>>
        + HasRelayTypeAt<1, 0, Relay = RelayTypeAt<Setup, B, A>>,
{
    async fn setup_birelay(
        setup: &Setup,
        _index: Twindex<A, B>,
        chain_a: &ChainTypeAt<Setup, A>,
        chain_b: &ChainTypeAt<Setup, B>,
        client_id_a: &ClientIdOf<ChainTypeAt<Setup, A>, ChainTypeAt<Setup, B>>,
        client_id_b: &ClientIdOf<ChainTypeAt<Setup, B>, ChainTypeAt<Setup, A>>,
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

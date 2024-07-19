use cgp_core::error::{CanRaiseError, ErrorOf};
use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainTypeAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, RelayTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

use crate::setup::traits::builder_at::HasBuilderAt;
use crate::setup::traits::relay::RelaySetup;

pub struct SetupRelayWithBuilder;

impl<Setup, const A: usize, const B: usize> RelaySetup<Setup, A, B> for SetupRelayWithBuilder
where
    Setup: HasRelayTypeAt<A, B> + HasBuilderAt<A, B> + CanRaiseError<ErrorOf<Setup::Builder>>,
    ChainTypeAt<Setup, A>: HasIbcChainTypes<ChainTypeAt<Setup, B>> + Clone,
    ChainTypeAt<Setup, B>: HasIbcChainTypes<ChainTypeAt<Setup, A>> + Clone,
    Setup::Builder: CanBuildRelayFromChains<0, 1>
        + CanBuildRelayFromChains<1, 0>
        + HasChainTypeAt<0, Chain = ChainTypeAt<Setup, A>>
        + HasChainTypeAt<1, Chain = ChainTypeAt<Setup, B>>
        + HasRelayTypeAt<0, 1, Relay = RelayTypeAt<Setup, A, B>>
        + HasRelayTypeAt<1, 0, Relay = RelayTypeAt<Setup, B, A>>,
{
    async fn setup_relays(
        setup: &Setup,
        _index: Twindex<A, B>,
        chain_a: &ChainTypeAt<Setup, A>,
        chain_b: &ChainTypeAt<Setup, B>,
        client_id_a: &ClientIdOf<ChainTypeAt<Setup, A>, ChainTypeAt<Setup, B>>,
        client_id_b: &ClientIdOf<ChainTypeAt<Setup, B>, ChainTypeAt<Setup, A>>,
    ) -> Result<(RelayTypeAt<Setup, A, B>, RelayTypeAt<Setup, B, A>), Setup::Error> {
        let build = setup.builder();

        let relay_a_to_b = build
            .build_relay_from_chains(
                Twindex::<0, 1>,
                client_id_a,
                client_id_b,
                chain_a.clone(),
                chain_b.clone(),
            )
            .await
            .map_err(Setup::raise_error)?;

        let relay_b_to_a = build
            .build_relay_from_chains(
                Twindex::<1, 0>,
                client_id_b,
                client_id_a,
                chain_b.clone(),
                chain_a.clone(),
            )
            .await
            .map_err(Setup::raise_error)?;

        Ok((relay_a_to_b, relay_b_to_a))
    }
}

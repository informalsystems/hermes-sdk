use cgp_core::CanRaiseError;
use hermes_relayer_components::build::traits::components::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::runtime::types::aliases::ErrorOf;

use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::relay_at::{HasRelayTypeAt, RelayTypeAt};
use crate::setup::traits::builder_at::HasBuilderAt;
use crate::setup::traits::relay::RelaySetup;
use crate::types::index::Twindex;

pub struct SetupRelayWithBuilder;

impl<Setup, const A: usize, const B: usize> RelaySetup<Setup, A, B> for SetupRelayWithBuilder
where
    Setup: HasRelayTypeAt<A, B> + HasBuilderAt<A, B> + CanRaiseError<ErrorOf<Setup::Builder>>,
    ChainTypeAt<Setup, A>: HasIbcChainTypes<ChainTypeAt<Setup, B>> + Clone,
    ChainTypeAt<Setup, B>: HasIbcChainTypes<ChainTypeAt<Setup, A>> + Clone,
    Setup::Builder:
        CanBuildRelayFromChains<RelayAToBTarget> + CanBuildRelayFromChains<RelayBToATarget>,
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
                RelayAToBTarget,
                client_id_a,
                client_id_b,
                chain_a.clone(),
                chain_b.clone(),
            )
            .await
            .map_err(Setup::raise_error)?;

        let relay_b_to_a = build
            .build_relay_from_chains(
                RelayBToATarget,
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

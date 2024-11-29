use cgp::core::error::{CanRaiseError, ErrorOf};
use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{
    HasBoundedRelayTypeAt, HasRelayTypeAt, RelayAt,
};
use hermes_relayer_components::multi::types::index::Twindex;

use crate::setup::traits::builder_at::HasBuilderAt;
use crate::setup::traits::relay::RelaySetup;

pub struct SetupRelayWithBuilder;

impl<Setup, const A: usize, const B: usize> RelaySetup<Setup, A, B> for SetupRelayWithBuilder
where
    Setup: HasBoundedRelayTypeAt<A, B>
        + HasBoundedRelayTypeAt<B, A>
        + HasBuilderAt<A, B>
        + CanRaiseError<ErrorOf<Setup::Builder>>,
    ChainAt<Setup, A>: HasIbcChainTypes<ChainAt<Setup, B>> + Clone,
    ChainAt<Setup, B>: HasIbcChainTypes<ChainAt<Setup, A>> + Clone,
    Setup::Builder: CanBuildRelayFromChains<0, 1>
        + CanBuildRelayFromChains<1, 0>
        + HasChainTypeAt<0, Chain = ChainAt<Setup, A>>
        + HasChainTypeAt<1, Chain = ChainAt<Setup, B>>
        + HasRelayTypeAt<0, 1, Relay = RelayAt<Setup, A, B>>
        + HasRelayTypeAt<1, 0, Relay = RelayAt<Setup, B, A>>,
{
    async fn setup_relays(
        setup: &Setup,
        _index: Twindex<A, B>,
        chain_a: &ChainAt<Setup, A>,
        chain_b: &ChainAt<Setup, B>,
        client_id_a: &ClientIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>,
        client_id_b: &ClientIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>,
    ) -> Result<(RelayAt<Setup, A, B>, RelayAt<Setup, B, A>), Setup::Error> {
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

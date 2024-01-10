use cgp_core::CanRaiseError;
use hermes_relayer_components::build::traits::components::relay_from_chains_builder::CanBuildRelayFromChains;
use hermes_relayer_components::build::traits::target::relay::RelayAToBTarget;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientId;

use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::relay_at::{HasRelayTypeAt, RelayTypeAt};
use crate::setup::traits::builder_at::HasBuilderAt;
use crate::setup::traits::relay::RelaySetup;
use crate::types::error::ErrorOf;
use crate::types::index::Twindex;

pub struct SetupRelayWithBuilder;

impl<Setup, const A: usize, const B: usize> RelaySetup<Setup, A, B> for SetupRelayWithBuilder
where
    Setup: HasRelayTypeAt<A, B> + HasBuilderAt<A, B> + CanRaiseError<ErrorOf<Setup::Builder>>,
    ChainTypeAt<Setup, A>: HasIbcChainTypes<ChainTypeAt<Setup, B>> + Clone,
    ChainTypeAt<Setup, B>: HasIbcChainTypes<ChainTypeAt<Setup, A>> + Clone,
    Setup::Builder: CanBuildRelayFromChains<RelayAToBTarget>,
{
    async fn setup_relay(
        setup: &Setup,
        _index: Twindex<A, B>,
        chain_a: &ChainTypeAt<Setup, A>,
        chain_b: &ChainTypeAt<Setup, B>,
        client_id_a: &ClientId<ChainTypeAt<Setup, A>, ChainTypeAt<Setup, B>>,
        client_id_b: &ClientId<ChainTypeAt<Setup, B>, ChainTypeAt<Setup, A>>,
    ) -> Result<RelayTypeAt<Setup, A, B>, Setup::Error> {
        let relay = setup
            .builder()
            .build_relay_from_chains(
                RelayAToBTarget,
                client_id_a,
                client_id_b,
                chain_a.clone(),
                chain_b.clone(),
            )
            .await
            .map_err(Setup::raise_error)?;

        Ok(relay)
    }
}

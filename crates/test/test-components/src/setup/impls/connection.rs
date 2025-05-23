use cgp::core::error::ErrorOf;
use cgp::core::field::Index;
use hermes_prelude::*;
use hermes_relayer_components::birelay::traits::HasTwoWayRelay;
use hermes_relayer_components::chain::traits::{HasIbcChainTypes, HasInitConnectionOptionsType};
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::{
    HasBoundedRelayTypeAt, HasRelayTypeAt, RelayAt,
};
use hermes_relayer_components::relay::impls::CanBootstrapConnection;

use crate::setup::traits::{ConnectionSetup, ConnectionSetupComponent, HasInitConnectionOptionsAt};

pub struct SetupConnectionHandshake;

#[cgp_provider(ConnectionSetupComponent)]
impl<Setup, A: Async, B: Async> ConnectionSetup<Setup, A, B> for SetupConnectionHandshake
where
    Setup: HasBiRelayTypeAt<A, B>
        + HasBoundedRelayTypeAt<A, B>
        + HasBoundedRelayTypeAt<B, A>
        + HasInitConnectionOptionsAt<A, B>
        + CanRaiseAsyncError<ErrorOf<RelayAt<Setup, A, B>>>,
    ChainAt<Setup, A>:
        HasIbcChainTypes<ChainAt<Setup, B>> + HasInitConnectionOptionsType<ChainAt<Setup, B>>,
    ChainAt<Setup, B>: HasIbcChainTypes<ChainAt<Setup, A>>,
    RelayAt<Setup, A, B>: CanBootstrapConnection,
    Setup::BiRelay:
        HasTwoWayRelay + HasRelayTypeAt<Index<0>, Index<1>, Relay = RelayAt<Setup, A, B>>,
{
    async fn setup_connection(
        setup: &Setup,
        birelay: &BiRelayAt<Setup, A, B>,
    ) -> Result<
        (
            ConnectionIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>,
            ConnectionIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>,
        ),
        Setup::Error,
    > {
        let (connection_id_a, connection_id_b) = birelay
            .relay_a_to_b()
            .bootstrap_connection(setup.init_connection_options())
            .await
            .map_err(Setup::raise_error)?;

        Ok((connection_id_a, connection_id_b))
    }
}

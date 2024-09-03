use cgp::core::error::{CanRaiseError, ErrorOf};
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, RelayAt};
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;

use crate::setup::traits::connection::ConnectionSetup;
use crate::setup::traits::init_connection_options_at::HasInitConnectionOptionsAt;

pub struct SetupConnectionHandshake;

impl<Setup, const A: usize, const B: usize> ConnectionSetup<Setup, A, B>
    for SetupConnectionHandshake
where
    Setup: HasBiRelayTypeAt<A, B>
        + HasInitConnectionOptionsAt<A, B>
        + CanRaiseError<ErrorOf<RelayAt<Setup, A, B>>>,
    ChainAt<Setup, A>:
        HasIbcChainTypes<ChainAt<Setup, B>> + HasInitConnectionOptionsType<ChainAt<Setup, B>>,
    ChainAt<Setup, B>: HasIbcChainTypes<ChainAt<Setup, A>>,
    RelayAt<Setup, A, B>: CanBootstrapConnection,
    BiRelayAt<Setup, A, B>: HasTwoWayRelay + HasRelayTypeAt<0, 1, Relay = RelayAt<Setup, A, B>>,
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
            .bootstrap_connection(&setup.init_connection_options())
            .await
            .map_err(Setup::raise_error)?;

        Ok((connection_id_a, connection_id_b))
    }
}

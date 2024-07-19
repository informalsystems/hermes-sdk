use cgp_core::error::{CanRaiseError, ErrorOf};
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, RelayTypeAt};
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;

use crate::setup::traits::connection::ConnectionSetup;
use crate::setup::traits::init_connection_options_at::HasInitConnectionOptionsAt;

pub struct SetupConnectionHandshake;

impl<Setup, const A: usize, const B: usize> ConnectionSetup<Setup, A, B>
    for SetupConnectionHandshake
where
    Setup: HasBiRelayTypeAt<A, B>
        + HasInitConnectionOptionsAt<A, B>
        + CanRaiseError<ErrorOf<RelayTypeAt<Setup, A, B>>>,
    ChainTypeAt<Setup, A>: HasIbcChainTypes<ChainTypeAt<Setup, B>>
        + HasInitConnectionOptionsType<ChainTypeAt<Setup, B>>,
    ChainTypeAt<Setup, B>: HasIbcChainTypes<ChainTypeAt<Setup, A>>,
    RelayTypeAt<Setup, A, B>: CanBootstrapConnection,
    BiRelayTypeAt<Setup, A, B>:
        HasTwoWayRelay + HasRelayTypeAt<0, 1, Relay = RelayTypeAt<Setup, A, B>>,
{
    async fn setup_connection(
        setup: &Setup,
        birelay: &BiRelayTypeAt<Setup, A, B>,
    ) -> Result<
        (
            ConnectionIdOf<ChainTypeAt<Setup, A>, ChainTypeAt<Setup, B>>,
            ConnectionIdOf<ChainTypeAt<Setup, B>, ChainTypeAt<Setup, A>>,
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

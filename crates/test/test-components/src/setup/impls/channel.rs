use cgp_core::CanRaiseError;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelId, ConnectionId};
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::relay_at::RelayTypeAt;
use crate::setup::traits::channel::ChannelSetup;
use crate::setup::traits::init_channel_options_at::HasInitChannelOptionsAt;
use crate::setup::traits::port_id_at::HasPortIdAt;
use crate::types::error::ErrorOf;
use crate::types::index::Twindex;

pub struct SetupChannel;

impl<Setup, const A: usize, const B: usize> ChannelSetup<Setup, A, B> for SetupChannel
where
    Setup: HasBiRelayTypeAt<A, B>
        + HasInitChannelOptionsAt<A, B>
        + HasPortIdAt<A, B>
        + HasPortIdAt<B, A>
        + CanRaiseError<ErrorOf<RelayTypeAt<Setup, A, B>>>,
    ChainTypeAt<Setup, A>:
        HasIbcChainTypes<ChainTypeAt<Setup, B>> + HasInitChannelOptionsType<ChainTypeAt<Setup, B>>,
    ChainTypeAt<Setup, B>: HasIbcChainTypes<ChainTypeAt<Setup, A>>,
    RelayTypeAt<Setup, A, B>: CanBootstrapChannel,
    BiRelayTypeAt<Setup, A, B>: HasTwoWayRelay,
{
    async fn setup_channel(
        setup: &Setup,
        birelay: &BiRelayTypeAt<Setup, A, B>,
        connection_id_a: &ConnectionId<ChainTypeAt<Setup, A>, ChainTypeAt<Setup, B>>,
        connection_id_b: &ConnectionId<ChainTypeAt<Setup, B>, ChainTypeAt<Setup, A>>,
    ) -> Result<
        (
            ChannelId<ChainTypeAt<Setup, A>, ChainTypeAt<Setup, B>>,
            ChannelId<ChainTypeAt<Setup, B>, ChainTypeAt<Setup, A>>,
        ),
        Setup::Error,
    > {
        let (channel_id_a, channel_id_b) = birelay
            .relay_a_to_b()
            .bootstrap_channel(
                setup.port_id_at(Twindex::<A, B>),
                setup.port_id_at(Twindex::<B, A>),
                setup.init_channel_options(connection_id_a, connection_id_b),
            )
            .await
            .map_err(Setup::raise_error)?;

        Ok((channel_id_a, channel_id_b))
    }
}

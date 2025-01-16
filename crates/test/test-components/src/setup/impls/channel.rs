use core::marker::PhantomData;

use cgp::core::error::{CanRaiseAsyncError, ErrorOf};
use cgp::core::field::Index;
use cgp::core::Async;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, ConnectionIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::{
    HasBoundedRelayTypeAt, HasRelayTypeAt, RelayAt,
};
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;

use crate::setup::traits::channel::ChannelSetup;
use crate::setup::traits::init_channel_options_at::HasInitChannelOptionsAt;
use crate::setup::traits::port_id_at::HasPortIdAt;

pub struct SetupChannelHandshake;

impl<Setup, A: Async, B: Async> ChannelSetup<Setup, A, B> for SetupChannelHandshake
where
    Setup: HasBiRelayTypeAt<A, B>
        + HasBoundedRelayTypeAt<A, B>
        + HasBoundedRelayTypeAt<B, A>
        + HasInitChannelOptionsAt<A, B>
        + HasPortIdAt<A, B>
        + HasPortIdAt<B, A>
        + CanRaiseAsyncError<ErrorOf<RelayAt<Setup, A, B>>>,
    ChainAt<Setup, A>:
        HasIbcChainTypes<ChainAt<Setup, B>> + HasInitChannelOptionsType<ChainAt<Setup, B>>,
    ChainAt<Setup, B>: HasIbcChainTypes<ChainAt<Setup, A>>,
    RelayAt<Setup, A, B>: CanBootstrapChannel,
    BiRelayAt<Setup, A, B>:
        HasTwoWayRelay + HasRelayTypeAt<Index<0>, Index<1>, Relay = RelayAt<Setup, A, B>>,
    PortIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>: Clone,
    PortIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>: Clone,
{
    async fn setup_channel(
        setup: &Setup,
        birelay: &BiRelayAt<Setup, A, B>,
        connection_id_a: &ConnectionIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>,
        connection_id_b: &ConnectionIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>,
    ) -> Result<
        (
            ChannelIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>,
            ChannelIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>,
            PortIdOf<ChainAt<Setup, A>, ChainAt<Setup, B>>,
            PortIdOf<ChainAt<Setup, B>, ChainAt<Setup, A>>,
        ),
        Setup::Error,
    > {
        let port_id_a = setup.port_id_at(PhantomData::<(A, B)>);
        let port_id_b = setup.port_id_at(PhantomData::<(B, A)>);

        let (channel_id_a, channel_id_b) = birelay
            .relay_a_to_b()
            .bootstrap_channel(
                port_id_a,
                port_id_b,
                &setup.init_channel_options(connection_id_a, connection_id_b),
            )
            .await
            .map_err(Setup::raise_error)?;

        Ok((
            channel_id_a,
            channel_id_b,
            port_id_a.clone(),
            port_id_b.clone(),
        ))
    }
}

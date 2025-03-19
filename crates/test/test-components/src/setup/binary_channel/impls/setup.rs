use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;

use crate::chain_driver::traits::types::chain::HasChain;
use crate::driver::traits::types::chain_driver_at::ChainDriverAt;
use crate::setup::traits::birelay::CanSetupBiRelay;
use crate::setup::traits::chain::CanSetupChain;
use crate::setup::traits::channel::CanSetupChannel;
use crate::setup::traits::clients::CanSetupClients;
use crate::setup::traits::connection::CanSetupConnection;
use crate::setup::traits::driver::{DriverBuilder, DriverBuilderComponent, HasTestDriverType};
use crate::setup::traits::drivers::binary_channel::CanBuildTestDriverWithBinaryChannel;

pub struct SetupBinaryChannelDriver;

#[cgp_provider(DriverBuilderComponent)]
impl<Setup, ChainA, ChainB> DriverBuilder<Setup> for SetupBinaryChannelDriver
where
    Setup: HasTestDriverType
        + HasAsyncErrorType
        + CanSetupChain<Index<0>>
        + CanSetupChain<Index<1>>
        + HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + CanSetupClients<Index<0>, Index<1>>
        + CanSetupBiRelay<Index<0>, Index<1>>
        + CanSetupConnection<Index<0>, Index<1>>
        + CanSetupChannel<Index<0>, Index<1>>
        + CanBuildTestDriverWithBinaryChannel,
    ChainDriverAt<Setup, Index<0>>: HasChain<Chain = ChainA>,
    ChainDriverAt<Setup, Index<1>>: HasChain<Chain = ChainB>,
    ChainA: HasIbcChainTypes<ChainB> + HasAsyncErrorType,
    ChainB: HasIbcChainTypes<ChainA> + HasAsyncErrorType,
{
    async fn build_driver(setup: &Setup) -> Result<Setup::TestDriver, Setup::Error> {
        let chain_driver_a = setup.setup_chain(PhantomData::<Index<0>>).await?;

        let chain_driver_b = setup.setup_chain(PhantomData::<Index<1>>).await?;

        let chain_a = chain_driver_a.chain();

        let chain_b = chain_driver_b.chain();

        let (client_id_a, client_id_b) = setup.setup_clients(chain_a, chain_b).await?;

        let birelay = setup
            .setup_birelay(
                PhantomData::<(Index<0>, Index<1>)>,
                chain_a,
                chain_b,
                &client_id_a,
                &client_id_b,
            )
            .await?;

        let (connection_id_a, connection_id_b) = setup.setup_connection(&birelay).await?;

        let (channel_id_a, channel_id_b, port_id_a, port_id_b) = setup
            .setup_channel(&birelay, &connection_id_a, &connection_id_b)
            .await?;

        let driver = setup
            .build_driver_with_binary_channel(
                birelay,
                chain_driver_a,
                chain_driver_b,
                connection_id_a,
                connection_id_b,
                channel_id_a,
                channel_id_b,
                port_id_a,
                port_id_b,
            )
            .await?;

        Ok(driver)
    }
}

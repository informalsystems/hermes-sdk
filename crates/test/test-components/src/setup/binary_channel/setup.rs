use cgp_core::error::HasErrorType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use crate::chain_driver::traits::types::chain::HasChain;
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::chain_driver_at::ChainDriverTypeAt;
use crate::setup::traits::birelay::CanSetupBiRelay;
use crate::setup::traits::chain::CanSetupChain;
use crate::setup::traits::channel::CanSetupChannel;
use crate::setup::traits::clients::CanSetupClients;
use crate::setup::traits::connection::CanSetupConnection;
use crate::setup::traits::driver::{DriverBuilder, HasTestDriverType};
use crate::setup::traits::drivers::binary_channel::CanBuildTestDriverWithBinaryChannel;
use crate::types::index::{Index, Twindex};

pub struct SetupBinaryChannelDriver;

impl<Setup> DriverBuilder<Setup> for SetupBinaryChannelDriver
where
    Setup: HasTestDriverType
        + HasErrorType
        + CanSetupChain<0>
        + CanSetupChain<1>
        + CanSetupClients<0, 1>
        + CanSetupBiRelay<0, 1>
        + CanSetupConnection<0, 1>
        + CanSetupChannel<0, 1>
        + CanBuildTestDriverWithBinaryChannel,
    ChainDriverTypeAt<Setup, 0>: HasChain,
    ChainDriverTypeAt<Setup, 1>: HasChain,
    ChainTypeAt<Setup, 0>: HasIbcChainTypes<ChainTypeAt<Setup, 1>>,
    ChainTypeAt<Setup, 1>: HasIbcChainTypes<ChainTypeAt<Setup, 0>>,
{
    async fn build_driver(setup: &Setup) -> Result<Setup::TestDriver, Setup::Error> {
        let chain_driver_a = setup.setup_chain(Index::<0>).await?;

        let chain_driver_b = setup.setup_chain(Index::<1>).await?;

        let chain_a = chain_driver_a.chain();

        let chain_b = chain_driver_b.chain();

        let (client_id_a, client_id_b) = setup.setup_clients(chain_a, chain_b).await?;

        let birelay = setup
            .setup_birelay(
                Twindex::<0, 1>,
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

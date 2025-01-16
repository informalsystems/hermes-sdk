use cgp::core::field::Index;
use cgp::prelude::HasAsyncErrorType;
use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_test_components::driver::traits::types::chain_driver_at::HasChainDriverTypeAt;
use hermes_test_components::setup::traits::driver::HasTestDriverType;
use hermes_test_components::setup::traits::drivers::binary_channel::BinaryChannelDriverBuilder;
use ibc::core::host::types::identifiers::{ChannelId, ConnectionId, PortId};

use crate::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;

pub struct BuildCosmosBinaryChannelDriver;

impl<Setup> BinaryChannelDriverBuilder<Setup> for BuildCosmosBinaryChannelDriver
where
    Setup: HasBiRelayTypeAt<Index<0>, Index<1>, BiRelay = CosmosBiRelay>
        + HasRelayTypeAt<Index<0>, Index<1>, Relay = CosmosRelay>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = CosmosRelay>
        + HasChainTypeAt<Index<0>, Chain = CosmosChain>
        + HasChainTypeAt<Index<1>, Chain = CosmosChain>
        + HasChainDriverTypeAt<Index<0>, ChainDriver = CosmosChainDriver>
        + HasChainDriverTypeAt<Index<1>, ChainDriver = CosmosChainDriver>
        + HasTestDriverType<TestDriver = CosmosBinaryChannelTestDriver>
        + HasAsyncErrorType,
{
    async fn build_driver_with_binary_channel(
        _setup: &Setup,
        birelay: CosmosBiRelay,
        chain_driver_a: CosmosChainDriver,
        chain_driver_b: CosmosChainDriver,
        connection_id_a: ConnectionId,
        connection_id_b: ConnectionId,
        channel_id_a: ChannelId,
        channel_id_b: ChannelId,
        port_id_a: PortId,
        port_id_b: PortId,
    ) -> Result<CosmosBinaryChannelTestDriver, Setup::Error> {
        let relay_driver = CosmosRelayDriver { birelay };

        let driver = CosmosBinaryChannelTestDriver {
            relay_driver,
            chain_driver_a,
            chain_driver_b,
            connection_id_a,
            connection_id_b,
            channel_id_a,
            channel_id_b,
            port_id_a,
            port_id_b,
        };

        Ok(driver)
    }
}

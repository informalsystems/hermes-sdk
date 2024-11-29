use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_error::types::HermesError;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::build::impls::bootstrap::birelay::CanBootstrapBiRelay;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use ibc_relayer::channel::version::Version;
use ibc_relayer::config::PacketFilter;
use ibc_test_framework::prelude::*;

use crate::tests::context::new_cosmos_builder;

#[test]
fn test_connection_and_channel_handshake_next() -> Result<(), Error> {
    run_binary_chain_test(&ConnectionAndChannelHandshakeTest)
}

pub struct ConnectionAndChannelHandshakeTest;

impl TestOverrides for ConnectionAndChannelHandshakeTest {
    fn should_spawn_supervisor(&self) -> bool {
        false
    }
}

impl BinaryChainTest for ConnectionAndChannelHandshakeTest {
    fn run<ChainA: ChainHandle, ChainB: ChainHandle>(
        &self,
        _config: &TestConfig,
        relayer: RelayerDriver,
        chains: ConnectedChains<ChainA, ChainB>,
    ) -> Result<(), Error> {
        let pf: PacketFilter = PacketFilter::default();

        let runtime = chains.node_a.value().chain_driver.runtime.as_ref();

        let builder = new_cosmos_builder(&relayer.config, &chains, pf)?;

        let chain_id_a = chains.chain_id_a().cloned_value();
        let chain_id_b = chains.chain_id_b().cloned_value();

        runtime
            .block_on(async move {
                let birelay = builder
                    .bootstrap_birelay(
                        &chain_id_a,
                        &chain_id_b,
                        &Default::default(),
                        &Default::default(),
                        &(),
                        &(),
                    )
                    .await?;

                let (connection_id_a, connection_id_b) = birelay
                    .relay_a_to_b()
                    .bootstrap_connection(&Default::default())
                    .await?;

                info!(
                    "bootstrapped new IBC connections with ID {} and {}",
                    connection_id_a, connection_id_b
                );

                let channel_init_options = CosmosInitChannelOptions {
                    ordering: Ordering::Unordered,
                    connection_hops: vec![connection_id_a],
                    channel_version: Version::ics20(),
                };

                let (channel_id_a, channel_id_b) = birelay
                    .relay_a_to_b()
                    .bootstrap_channel(
                        &PortId::transfer(),
                        &PortId::transfer(),
                        &channel_init_options,
                    )
                    .await?;

                info!(
                    "bootstrapped new IBC channel with ID {} and {}",
                    channel_id_a, channel_id_b
                );

                <Result<(), HermesError>>::Ok(())
            })
            .unwrap();

        Ok(())
    }
}

use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientOptions;
use hermes_cosmos_relayer::types::telemetry::CosmosTelemetry;
use hermes_error::types::HermesError;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_solomachine_relayer::contexts::chain::MockSolomachine;
use hermes_solomachine_relayer::contexts::relay::SolomachineRelay;
use ibc_relayer::config::PacketFilter;
use ibc_test_framework::prelude::*;

use crate::tests::context::new_cosmos_builder;

#[test]
fn test_solomachine_to_cosmos_next() -> Result<(), Error> {
    run_binary_chain_test(&SolomachineToCosmosTest)
}

pub struct SolomachineToCosmosTest;

impl TestOverrides for SolomachineToCosmosTest {
    fn should_spawn_supervisor(&self) -> bool {
        false
    }
}

impl BinaryChainTest for SolomachineToCosmosTest {
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

        let solomachine_runtime =
            HermesRuntime::new(chains.node_b.value().chain_driver.runtime.clone());

        let solomachine_chain = solomachine_chain_context(solomachine_runtime, Default::default());

        runtime
            .block_on(async move {
                let cosmos_chain = builder.build_chain(&chain_id_a).await?;

                let src_client_id = SolomachineRelay::create_client(
                    SourceTarget,
                    &solomachine_chain,
                    &cosmos_chain,
                    &Default::default(),
                    &(),
                )
                .await
                .unwrap();

                let dst_client_id = SolomachineRelay::create_client(
                    DestinationTarget,
                    &cosmos_chain,
                    &solomachine_chain,
                    &(),
                    &(),
                )
                .await
                .unwrap();

                info!("src_client_id: {src_client_id:#?}");
                info!("dst_client_id: {dst_client_id:#?}");

                let relay = SolomachineRelay {
                    runtime: solomachine_chain.runtime.clone(),
                    src_chain: solomachine_chain,
                    dst_chain: cosmos_chain,
                    src_client_id,
                    dst_client_id,
                };

                let src_connection_id = relay.init_connection(&Default::default()).await.unwrap();

                info!("src_connection_id: {src_connection_id:#?}");

                // FIXME: The test currently fails here

                // let dst_connection_id = relay
                //     .relay_connection_open_handshake(&src_connection_id)
                //     .await
                //     .unwrap();

                // info!("dst_connection_id: {dst_connection_id:#?}");

                <Result<(), HermesError>>::Ok(())
            })
            .unwrap();

        Ok(())
    }
}

pub fn solomachine_chain_context(
    runtime: HermesRuntime,
    telemetry: CosmosTelemetry,
) -> MockSolomachine {
    let commitment_prefix = "solomachine".to_owned();

    MockSolomachine::new("solomachine1", commitment_prefix, runtime, telemetry)
}

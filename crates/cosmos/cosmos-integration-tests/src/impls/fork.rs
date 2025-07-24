use alloc::sync::Arc;
use std::path::PathBuf;

use cgp::extra::runtime::HasRuntime;
use hermes_core::runtime_components::traits::{CanCreateDir, CanExecCommand, CanSleep};
use hermes_core::test_components::setup::traits::{FullNodeForker, FullNodeForkerComponent};
use hermes_cosmos_core::test_components::bootstrap::traits::CanStartChainFullNodes;
use hermes_cosmos_relayer::contexts::CosmosBuilder;
use hermes_error::HermesError;
use hermes_prelude::*;

use crate::contexts::{
    CosmosBinaryChannelTestDriver, CosmosBootstrap, CosmosBootstrapFields, CosmosChainDriver,
};

#[cgp_new_provider(FullNodeForkerComponent)]
impl FullNodeForker<CosmosBinaryChannelTestDriver> for ForkSecondFullNode {
    async fn fork_full_node(
        driver: &CosmosBinaryChannelTestDriver,
    ) -> Result<CosmosBinaryChannelTestDriver, HermesError> {
        // Retrieve necessary full node data
        let genesis_config = driver.chain_driver_b.genesis_config.clone();
        let chain_node_config = driver.chain_driver_b.chain_node_config.clone();
        let chain_home_dir = driver
            .chain_driver_b
            .chain_node_config
            .chain_home_dir
            .clone();

        let runtime = driver.chain_driver_b.chain.runtime.clone();
        let builder = CosmosBuilder::new_with_default(runtime.clone());

        let node_bootstrap = CosmosBootstrap {
            fields: Arc::new(CosmosBootstrapFields {
                runtime: runtime.clone(),
                cosmos_builder: builder.clone(),
                should_randomize_identifiers: true,
                chain_store_dir: chain_home_dir.clone(),
                chain_command_path: driver.chain_driver_b.chain_command_path.clone(),
                account_prefix: driver
                    .chain_driver_b
                    .chain
                    .chain_config
                    .account_prefix
                    .clone(),
                staking_denom_prefix: driver
                    .chain_driver_b
                    .genesis_config
                    .staking_denom
                    .to_string(),
                transfer_denom_prefix: driver
                    .chain_driver_b
                    .genesis_config
                    .transfer_denom
                    .to_string(),
                genesis_config_modifier: Box::new(|_| Ok(())),
                comet_config_modifier: Box::new(|_| Ok(())),
                dynamic_gas: driver
                    .chain_driver_b
                    .chain
                    .chain_config
                    .gas_config
                    .dynamic_gas_config
                    .clone(),
            }),
        };

        // Stop full node
        runtime
            .exec_command(
                &PathBuf::from("pkill".to_string()),
                &["-f", &driver.chain_driver_b.chain.chain_id.to_string()],
            )
            .await
            .unwrap();

        driver
            .relay_driver
            .birelay
            .runtime()
            .sleep(core::time::Duration::from_secs(5))
            .await;

        // Build forked full node data
        let fork_chain_home_dir = chain_home_dir
            .as_path()
            .parent()
            .unwrap()
            .join(format!("fork-{}", driver.chain_driver_b.chain.chain_id));
        let mut fork_chain_node_config = chain_node_config.clone();
        fork_chain_node_config.chain_home_dir = fork_chain_home_dir.clone();
        fork_chain_node_config.rpc_port += 1;
        fork_chain_node_config.p2p_port += 1;
        fork_chain_node_config.grpc_port += 1;
        let fork_rpc_port = fork_chain_node_config.rpc_port;
        let fork_p2p_port = fork_chain_node_config.p2p_port;

        let fork_bootstrap = CosmosBootstrap {
            fields: Arc::new(CosmosBootstrapFields {
                runtime: runtime.clone(),
                cosmos_builder: builder.clone(),
                should_randomize_identifiers: true,
                chain_store_dir: fork_chain_home_dir.clone(),
                chain_command_path: driver.chain_driver_b.chain_command_path.clone(),
                account_prefix: driver
                    .chain_driver_b
                    .chain
                    .chain_config
                    .account_prefix
                    .clone(),
                staking_denom_prefix: driver
                    .chain_driver_b
                    .genesis_config
                    .staking_denom
                    .to_string(),
                transfer_denom_prefix: driver
                    .chain_driver_b
                    .genesis_config
                    .transfer_denom
                    .to_string(),
                genesis_config_modifier: Box::new(|_| Ok(())),
                comet_config_modifier: Box::new(|_| Ok(())),
                dynamic_gas: driver
                    .chain_driver_b
                    .chain
                    .chain_config
                    .gas_config
                    .dynamic_gas_config
                    .clone(),
            }),
        };

        // Create forked full node directory and copy full node data inside
        runtime.create_dir(&fork_chain_home_dir).await.unwrap();

        // Copy data to fork
        copy_dir_recursive(&chain_home_dir, &fork_chain_home_dir);

        let fork_chain_config_path = fork_chain_home_dir.join("config").join("config.toml");

        let fork_chain_config = std::fs::read_to_string(fork_chain_config_path.clone())
            .expect("failed to read fork config.toml");

        let mut toml_value: toml::Table = fork_chain_config.parse()?;

        // Update RPC and P2P addresses to avoid collision
        toml_value
            .get_mut("rpc")
            .and_then(|rpc| rpc.as_table_mut())
            .expect("Failed to retrieve `rpc` in node configuration")
            .insert(
                "laddr".to_string(),
                toml::Value::String(format!("tcp://0.0.0.0:{fork_rpc_port}")),
            );
        toml_value
            .get_mut("p2p")
            .and_then(|p2p| p2p.as_table_mut())
            .expect("Failed to retrieve `p2p` in node configuration")
            .insert(
                "laddr".to_string(),
                toml::Value::String(format!("tcp://0.0.0.0:{fork_p2p_port}")),
            );

        std::fs::write(fork_chain_config_path, toml::to_string(&toml_value)?)?;

        // Start the forked chain full node in the background, and return the child process handle
        let mut chain_processes = fork_bootstrap
            .start_chain_full_nodes(
                &fork_chain_home_dir,
                &fork_chain_node_config,
                &genesis_config,
            )
            .await?;

        driver
            .relay_driver
            .birelay
            .runtime()
            .sleep(core::time::Duration::from_secs(1))
            .await;

        let mut node_chain_processes = node_bootstrap
            .start_chain_full_nodes(&chain_home_dir, &chain_node_config, &genesis_config)
            .await?;

        chain_processes.append(&mut node_chain_processes);

        let fork_chain_a_driver = CosmosChainDriver {
            chain: driver.chain_driver_a.chain.clone(),
            chain_command_path: driver.chain_driver_a.chain_command_path.clone(),
            chain_node_config: driver.chain_driver_a.chain_node_config.clone(),
            genesis_config: driver.chain_driver_a.genesis_config.clone(),
            chain_processes: vec![],
            validator_wallet: driver.chain_driver_a.validator_wallet.clone(),
            relayer_wallet: driver.chain_driver_a.relayer_wallet.clone(),
            user_wallet_a: driver.chain_driver_a.user_wallet_a.clone(),
            user_wallet_b: driver.chain_driver_a.user_wallet_b.clone(),
            wallets: driver.chain_driver_a.wallets.clone(),
        };

        let fork_chain_b_driver = CosmosChainDriver {
            chain: driver.chain_driver_b.chain.clone(),
            chain_command_path: driver.chain_driver_b.chain_command_path.clone(),
            chain_node_config: fork_chain_node_config,
            genesis_config,
            chain_processes,
            validator_wallet: driver.chain_driver_b.validator_wallet.clone(),
            relayer_wallet: driver.chain_driver_b.relayer_wallet.clone(),
            user_wallet_a: driver.chain_driver_b.user_wallet_a.clone(),
            user_wallet_b: driver.chain_driver_b.user_wallet_b.clone(),
            wallets: driver.chain_driver_b.wallets.clone(),
        };

        Ok(CosmosBinaryChannelTestDriver {
            relay_driver: driver.relay_driver.clone(),
            chain_driver_a: fork_chain_a_driver,
            chain_driver_b: fork_chain_b_driver,
            client_id_a: driver.client_id_a.clone(),
            client_id_b: driver.client_id_b.clone(),
            connection_id_a: driver.connection_id_a.clone(),
            connection_id_b: driver.connection_id_b.clone(),
            channel_id_a: driver.channel_id_a.clone(),
            channel_id_b: driver.channel_id_b.clone(),
            port_id_a: driver.port_id_a.clone(),
            port_id_b: driver.port_id_b.clone(),
            create_client_payload_options_a: driver.create_client_payload_options_a.clone(),
            create_client_payload_options_b: driver.create_client_payload_options_b.clone(),
            create_client_message_options_a: driver.create_client_message_options_a,
            create_client_message_options_b: driver.create_client_message_options_b,
            recover_client_payload_options_a: driver.recover_client_payload_options_a.clone(),
            recover_client_payload_options_b: driver.recover_client_payload_options_b.clone(),
        })
    }
}

fn copy_dir_recursive(source_dir: &PathBuf, destination_dir: &PathBuf) {
    if !destination_dir.exists() {
        std::fs::create_dir_all(destination_dir).expect("failed to create destination directory");
    }

    for entry in std::fs::read_dir(source_dir).expect("failed to read source directory") {
        let entry = entry.expect("failed to extract entry");
        let path = entry.path();
        let dest_path = destination_dir.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path);
        } else {
            std::fs::copy(&path, &dest_path).expect("failed to copy file recusively");
        }
    }
}

use core::str::FromStr;
use core::time::Duration;
use std::path::PathBuf;

use cgp::core::error::ErrorOf;
use cgp::extra::runtime::HasRuntime;
use cgp::prelude::*;
use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use hermes_cosmos_chain_components::types::config::gas::gas_config::{GasConfig, GasPrice};
use hermes_cosmos_chain_components::types::key_types::secp256k1::KEYSTORE_FILE_EXTENSION;
use hermes_cosmos_test_components::bootstrap::traits::fields::account_prefix::HasAccountPrefix;
use hermes_cosmos_test_components::bootstrap::traits::fields::dynamic_gas_fee::HasDynamicGas;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_runtime_components::traits::fs::create_dir::CanCreateDir;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_test_components::chain::traits::HasWalletType;
use hermes_test_components::chain_driver::traits::HasChainType;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use tendermint_rpc::{Error as TendermintRpcError, Url};

use crate::traits::bootstrap::compat_mode::HasCompatMode;
use crate::traits::bootstrap::relayer_chain_config::{
    RelayerChainConfigBuilder, RelayerChainConfigBuilderComponent,
};

pub struct BuildRelayerChainConfig;

#[cgp_provider(RelayerChainConfigBuilderComponent)]
impl<Bootstrap, Chain> RelayerChainConfigBuilder<Bootstrap> for BuildRelayerChainConfig
where
    Bootstrap: HasRuntime
        + HasAccountPrefix
        + HasCompatMode
        + HasDynamicGas
        + HasChainNodeConfigType<ChainNodeConfig = CosmosChainNodeConfig>
        + HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + HasChainType<Chain = Chain>
        + CanRaiseAsyncError<TendermintRpcError>
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<ErrorOf<Bootstrap::Runtime>>,
    Chain: HasWalletType<Wallet = CosmosTestWallet>,
    Bootstrap::Runtime: HasFilePathType<FilePath = PathBuf> + CanWriteStringToFile + CanCreateDir,
{
    async fn build_relayer_chain_config(
        bootstrap: &Bootstrap,
        chain_node_config: &CosmosChainNodeConfig,
        chain_genesis_config: &CosmosGenesisConfig,
        relayer_wallet: &CosmosTestWallet,
    ) -> Result<CosmosChainConfig, Bootstrap::Error> {
        let gas_multiplier = 1.3;
        let gas_price = 1.0;
        let fee_granter = "".to_owned();
        let max_gas = 900000000;
        let max_amount = (max_gas as f64 * gas_multiplier) * gas_price;
        let max_gas_fee_in_coin = Coin {
            denom: chain_genesis_config.staking_denom.to_string(),
            amount: max_amount.to_string(),
        };

        let max_fee = Fee {
            amount: vec![max_gas_fee_in_coin],
            gas_limit: max_gas,
            payer: "".to_string(),
            granter: fee_granter.clone(),
        };

        let gas_config = GasConfig {
            default_gas: 400_000,
            max_gas,
            gas_multiplier,
            gas_price: GasPrice::new(1.0, chain_genesis_config.staking_denom.to_string()),
            max_fee,
            fee_granter,
            dynamic_gas_config: bootstrap.dynamic_gas().clone(),
        };

        let keypair = &relayer_wallet.keypair;
        let key_name = relayer_wallet.id.clone();
        let key_store_folder = chain_node_config.chain_home_dir.join("hermes_keyring");

        {
            let runtime = bootstrap.runtime();

            runtime
                .create_dir(&key_store_folder)
                .await
                .map_err(Bootstrap::raise_error)?;

            let mut file_path = key_store_folder.join(key_name.clone());
            file_path.set_extension(KEYSTORE_FILE_EXTENSION);

            let keypair_str =
                serde_json::to_string_pretty(keypair).map_err(Bootstrap::raise_error)?;

            runtime
                .write_string_to_file(&file_path, &keypair_str)
                .await
                .map_err(Bootstrap::raise_error)?;
        }

        let relayer_chain_config = CosmosChainConfig {
            id: chain_node_config.chain_id.to_string(),
            rpc_addr: Url::from_str(&format!("http://localhost:{}", chain_node_config.rpc_port))
                .map_err(Bootstrap::raise_error)?,
            grpc_addr: Url::from_str(&format!("http://localhost:{}", chain_node_config.grpc_port))
                .map_err(Bootstrap::raise_error)?,
            rpc_timeout: Duration::from_secs(10),
            account_prefix: bootstrap.account_prefix().into(),
            key_name,
            key_store_folder: Some(key_store_folder),
            store_prefix: "ibc".to_string(),
            max_msg_num: Default::default(),
            max_tx_size: Default::default(),
            max_block_time: Duration::from_secs(30),
            clock_drift: Duration::from_secs(5),
            gas_config,
            address_type: "cosmos".to_string(),
            extension_options: Default::default(),
            compat_mode: bootstrap
                .compat_mode()
                .map(|compat_mode| compat_mode.to_string()),
            block_time: Duration::from_secs(1),
        };

        Ok(relayer_chain_config)
    }
}

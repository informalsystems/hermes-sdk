use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use std::path::PathBuf;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use eyre::eyre;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_cosmos_test_components::chain_driver::components::CosmosChainDriverComponents as BaseCosmosChainDriverComponents;
use hermes_cosmos_test_components::chain_driver::traits::grpc_port::GrpcPortGetter;
use hermes_cosmos_test_components::chain_driver::traits::rpc_port::RpcPortGetter;
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::Error;
use hermes_relayer_components::multi::types::index::Index;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use hermes_test_components::chain::traits::proposal::types::proposal_id::ProposalIdTypeComponent;
use hermes_test_components::chain::traits::proposal::types::proposal_status::ProposalStatusTypeComponent;
use hermes_test_components::chain_driver::traits::chain_process::ChainProcessTaker;
use hermes_test_components::chain_driver::traits::config::ConfigUpdater;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::ChainHomeDirGetter;
use hermes_test_components::chain_driver::traits::fields::denom_at::{
    DenomGetterAt, StakingDenom, TransferDenom,
};
use hermes_test_components::chain_driver::traits::fields::wallet::{
    RelayerWallet, UserWallet, ValidatorWallet, WalletGetterAt, WalletsGetter,
};
use hermes_test_components::chain_driver::traits::types::chain::{ChainGetter, ProvideChainType};
use ibc_relayer::config::{ChainConfig, Config};
use tokio::process::Child;
use tokio::sync::Mutex;
use toml::{to_string_pretty, Value};

/**
   A chain driver for adding test functionalities to a Cosmos chain.
*/
pub struct CosmosChainDriver {
    pub chain: CosmosChain,
    pub chain_command_path: PathBuf,
    pub chain_process: Arc<Mutex<Option<Child>>>,
    pub chain_node_config: CosmosChainNodeConfig,
    pub genesis_config: CosmosGenesisConfig,
    pub validator_wallet: CosmosTestWallet,
    pub relayer_wallet: CosmosTestWallet,
    pub user_wallet_a: CosmosTestWallet,
    pub user_wallet_b: CosmosTestWallet,
    pub wallets: BTreeMap<String, CosmosTestWallet>,
}

pub struct CosmosChainDriverComponents;

impl HasComponents for CosmosChainDriver {
    type Components = CosmosChainDriverComponents;
}

delegate_components! {
    CosmosChainDriverComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            RandomAmountGeneratorComponent,
            ProposalIdTypeComponent,
            ProposalStatusTypeComponent,
        ]:
            BaseCosmosChainDriverComponents,
    }
}

impl ProvideChainType<CosmosChainDriver> for CosmosChainDriverComponents {
    type Chain = CosmosChain;
}

impl ChainGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain(driver: &CosmosChainDriver) -> &CosmosChain {
        &driver.chain
    }
}

impl RuntimeGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn runtime(chain_driver: &CosmosChainDriver) -> &HermesRuntime {
        &chain_driver.chain.runtime
    }
}

impl ChainHomeDirGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain_home_dir(chain_driver: &CosmosChainDriver) -> &PathBuf {
        &chain_driver.chain_node_config.chain_home_dir
    }
}

impl RpcPortGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn rpc_port(chain_driver: &CosmosChainDriver) -> u16 {
        chain_driver.chain_node_config.rpc_port
    }
}

impl GrpcPortGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn grpc_port(chain_driver: &CosmosChainDriver) -> u16 {
        chain_driver.chain_node_config.grpc_port
    }
}

impl WalletGetterAt<CosmosChainDriver, RelayerWallet, 0> for CosmosChainDriverComponents {
    fn wallet_at(
        driver: &CosmosChainDriver,
        _kind: RelayerWallet,
        _index: Index<0>,
    ) -> &CosmosTestWallet {
        &driver.relayer_wallet
    }
}

impl WalletGetterAt<CosmosChainDriver, UserWallet, 0> for CosmosChainDriverComponents {
    fn wallet_at(
        driver: &CosmosChainDriver,
        _kind: UserWallet,
        _index: Index<0>,
    ) -> &CosmosTestWallet {
        &driver.user_wallet_a
    }
}

impl WalletGetterAt<CosmosChainDriver, UserWallet, 1> for CosmosChainDriverComponents {
    fn wallet_at(
        driver: &CosmosChainDriver,
        _kind: UserWallet,
        _index: Index<1>,
    ) -> &CosmosTestWallet {
        &driver.user_wallet_b
    }
}

impl WalletGetterAt<CosmosChainDriver, ValidatorWallet, 0> for CosmosChainDriverComponents {
    fn wallet_at(
        driver: &CosmosChainDriver,
        _kind: ValidatorWallet,
        _index: Index<0>,
    ) -> &CosmosTestWallet {
        &driver.validator_wallet
    }
}

impl WalletsGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn wallets(chain_driver: &CosmosChainDriver) -> &BTreeMap<String, CosmosTestWallet> {
        &chain_driver.wallets
    }
}

impl DenomGetterAt<CosmosChainDriver, TransferDenom, 0> for CosmosChainDriverComponents {
    fn denom_at(driver: &CosmosChainDriver, _kind: TransferDenom, _index: Index<0>) -> &Denom {
        &driver.genesis_config.transfer_denom
    }
}

impl DenomGetterAt<CosmosChainDriver, StakingDenom, 0> for CosmosChainDriverComponents {
    fn denom_at(driver: &CosmosChainDriver, _kind: StakingDenom, _index: Index<0>) -> &Denom {
        &driver.genesis_config.staking_denom
    }
}

impl ChainCommandPathGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain_command_path(driver: &CosmosChainDriver) -> &PathBuf {
        &driver.chain_command_path
    }
}

impl ChainProcessTaker<CosmosChainDriver> for CosmosChainDriverComponents {
    async fn take_chain_process(chain_driver: &CosmosChainDriver) -> Option<Child> {
        chain_driver.chain_process.lock().await.take()
    }
}

impl ConfigUpdater<CosmosChainDriver, Value> for CosmosChainDriverComponents {
    fn update_config(
        chain_driver: &CosmosChainDriver,
        config: &mut Value,
    ) -> Result<String, Error> {
        let chain_config = Value::try_from(&chain_driver.chain.chain_config)?;
        let chain_config_str = to_string_pretty(&chain_config)?;

        if let Some(chains_config) = config.get_mut("chains") {
            let chains_config = chains_config
                .as_array_mut()
                .ok_or_else(|| eyre!("expect chain entries as array"))?;

            chains_config.push(chain_config);
        } else {
            config
                .as_table_mut()
                .ok_or(eyre!("expect object"))?
                .insert("chains".into(), vec![chain_config].into());
        };

        Ok(chain_config_str)
    }
}

impl ConfigUpdater<CosmosChainDriver, Config> for CosmosChainDriverComponents {
    fn update_config(
        chain_driver: &CosmosChainDriver,
        config: &mut Config,
    ) -> Result<String, Error> {
        let chain_config_str = to_string_pretty(&chain_driver.chain.chain_config)?;

        let chain_config = chain_driver.chain.chain_config.clone();

        config.chains.push(ChainConfig::CosmosSdk(chain_config));

        Ok(chain_config_str)
    }
}

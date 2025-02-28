use alloc::collections::BTreeMap;
use core::marker::PhantomData;
use std::path::PathBuf;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_cosmos_chain_components::impls::types::config::RelayerConfig;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::{
    ChainCommandPathGetter, ChainCommandPathGetterComponent,
};
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_cosmos_test_components::chain_driver::components::CosmosChainDriverComponents as BaseCosmosChainDriverComponents;
use hermes_cosmos_test_components::chain_driver::traits::grpc_port::{
    GrpcPortGetter, GrpcPortGetterComponent,
};
use hermes_cosmos_test_components::chain_driver::traits::rpc_port::{
    RpcPortGetter, RpcPortGetterComponent,
};
use hermes_error::handlers::debug::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_error::types::Error;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetter, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_test_components::chain::traits::proposal::types::proposal_id::ProposalIdTypeComponent;
use hermes_test_components::chain::traits::proposal::types::proposal_status::ProposalStatusTypeComponent;
use hermes_test_components::chain_driver::traits::chain_process::{
    ChainProcessTaker, ChainProcessTakerComponent,
};
use hermes_test_components::chain_driver::traits::config::{ConfigUpdater, ConfigUpdaterComponent};
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::{
    ChainHomeDirGetter, ChainHomeDirGetterComponent,
};
use hermes_test_components::chain_driver::traits::fields::denom_at::{
    DenomGetterAt, DenomGetterComponent, StakingDenom, TransferDenom,
};
use hermes_test_components::chain_driver::traits::fields::wallet::{
    RelayerWallet, UserWallet, ValidatorWallet, WalletGetterAt, WalletGetterComponent,
    WalletsGetter, WalletsGetterComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainGetter, ChainGetterComponent, ChainTypeComponent, ProvideChainType,
};
use hermes_test_components::chain_driver::traits::wait::{
    CanWaitChainStartup, ChainStartupWaiterComponent,
};
use tokio::process::Child;
use toml::to_string_pretty;

/**
   A chain driver for adding test functionalities to a Cosmos chain.
*/
#[cgp_context(CosmosChainDriverComponents)]
pub struct CosmosChainDriver {
    pub chain: CosmosChain,
    pub chain_command_path: PathBuf,
    pub chain_process: Option<Child>,
    pub chain_node_config: CosmosChainNodeConfig,
    pub genesis_config: CosmosGenesisConfig,
    pub validator_wallet: CosmosTestWallet,
    pub relayer_wallet: CosmosTestWallet,
    pub user_wallet_a: CosmosTestWallet,
    pub user_wallet_b: CosmosTestWallet,
    pub wallets: BTreeMap<String, CosmosTestWallet>,
}

delegate_components! {
    CosmosChainDriverComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeProviderComponent:
            ProvideHermesRuntime,
        [
            RandomAmountGeneratorComponent,
            ChainStartupWaiterComponent,
            ProposalIdTypeComponent,
            ProposalStatusTypeComponent,
        ]:
            BaseCosmosChainDriverComponents,
    }
}

#[cgp_provider(ChainTypeComponent)]
impl ProvideChainType<CosmosChainDriver> for CosmosChainDriverComponents {
    type Chain = CosmosChain;
}

#[cgp_provider(ChainGetterComponent)]
impl ChainGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain(driver: &CosmosChainDriver) -> &CosmosChain {
        &driver.chain
    }
}

#[cgp_provider(RuntimeGetterComponent)]
impl RuntimeGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn runtime(chain_driver: &CosmosChainDriver) -> &HermesRuntime {
        &chain_driver.chain.runtime
    }
}

#[cgp_provider(ChainHomeDirGetterComponent)]
impl ChainHomeDirGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain_home_dir(chain_driver: &CosmosChainDriver) -> &PathBuf {
        &chain_driver.chain_node_config.chain_home_dir
    }
}

#[cgp_provider(RpcPortGetterComponent)]
impl RpcPortGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn rpc_port(chain_driver: &CosmosChainDriver) -> u16 {
        chain_driver.chain_node_config.rpc_port
    }
}

#[cgp_provider(GrpcPortGetterComponent)]
impl GrpcPortGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn grpc_port(chain_driver: &CosmosChainDriver) -> u16 {
        chain_driver.chain_node_config.grpc_port
    }
}

#[cgp_provider(WalletGetterComponent)]
impl WalletGetterAt<CosmosChainDriver, RelayerWallet, Index<0>> for CosmosChainDriverComponents {
    fn wallet_at(
        driver: &CosmosChainDriver,
        _kind: RelayerWallet,
        _index: PhantomData<Index<0>>,
    ) -> &CosmosTestWallet {
        &driver.relayer_wallet
    }
}

#[cgp_provider(WalletGetterComponent)]
impl WalletGetterAt<CosmosChainDriver, UserWallet, Index<0>> for CosmosChainDriverComponents {
    fn wallet_at(
        driver: &CosmosChainDriver,
        _kind: UserWallet,
        _index: PhantomData<Index<0>>,
    ) -> &CosmosTestWallet {
        &driver.user_wallet_a
    }
}

#[cgp_provider(WalletGetterComponent)]
impl WalletGetterAt<CosmosChainDriver, UserWallet, Index<1>> for CosmosChainDriverComponents {
    fn wallet_at(
        driver: &CosmosChainDriver,
        _kind: UserWallet,
        _index: PhantomData<Index<1>>,
    ) -> &CosmosTestWallet {
        &driver.user_wallet_b
    }
}

#[cgp_provider(WalletGetterComponent)]
impl WalletGetterAt<CosmosChainDriver, ValidatorWallet, Index<0>> for CosmosChainDriverComponents {
    fn wallet_at(
        driver: &CosmosChainDriver,
        _kind: ValidatorWallet,
        _index: PhantomData<Index<0>>,
    ) -> &CosmosTestWallet {
        &driver.validator_wallet
    }
}

#[cgp_provider(WalletsGetterComponent)]
impl WalletsGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn wallets(chain_driver: &CosmosChainDriver) -> &BTreeMap<String, CosmosTestWallet> {
        &chain_driver.wallets
    }
}

#[cgp_provider(DenomGetterComponent)]
impl DenomGetterAt<CosmosChainDriver, TransferDenom, Index<0>> for CosmosChainDriverComponents {
    fn denom_at(
        driver: &CosmosChainDriver,
        _kind: TransferDenom,
        _index: PhantomData<Index<0>>,
    ) -> &Denom {
        &driver.genesis_config.transfer_denom
    }
}

#[cgp_provider(DenomGetterComponent)]
impl DenomGetterAt<CosmosChainDriver, StakingDenom, Index<0>> for CosmosChainDriverComponents {
    fn denom_at(
        driver: &CosmosChainDriver,
        _kind: StakingDenom,
        _index: PhantomData<Index<0>>,
    ) -> &Denom {
        &driver.genesis_config.staking_denom
    }
}

#[cgp_provider(ChainCommandPathGetterComponent)]
impl ChainCommandPathGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain_command_path(driver: &CosmosChainDriver) -> &PathBuf {
        &driver.chain_command_path
    }
}

#[cgp_provider(ChainProcessTakerComponent)]
impl ChainProcessTaker<CosmosChainDriver> for CosmosChainDriverComponents {
    fn take_chain_process(chain_driver: &mut CosmosChainDriver) -> Option<Child> {
        chain_driver.chain_process.take()
    }
}

#[cgp_provider(ConfigUpdaterComponent)]
impl ConfigUpdater<CosmosChainDriver, RelayerConfig> for CosmosChainDriverComponents {
    fn update_config(
        chain_driver: &CosmosChainDriver,
        config: &mut RelayerConfig,
    ) -> Result<String, Error> {
        let chain_config_str = to_string_pretty(&chain_driver.chain.chain_config)?;

        let chain_config = chain_driver.chain.chain_config.clone();

        config.chains.push(chain_config);

        Ok(chain_config_str)
    }
}

pub trait CanUseCosmosChainDriver: CanWaitChainStartup {}

impl CanUseCosmosChainDriver for CosmosChainDriver {}

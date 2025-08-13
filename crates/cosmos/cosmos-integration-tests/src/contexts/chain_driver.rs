use alloc::collections::BTreeMap;
use core::marker::PhantomData;
use std::path::PathBuf;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::runtime_components::traits::{
    RuntimeGetter, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_core::test_components::chain::traits::{
    ProposalIdTypeComponent, ProposalStatusTypeComponent,
};
use hermes_core::test_components::chain_driver::traits::{
    ChainGetter, ChainGetterComponent, ChainHomeDirGetter, ChainHomeDirGetterComponent,
    ChainProcessTaker, ChainProcessTakerComponent, ChainStartupWaiterComponent, ChainTypeProvider,
    ChainTypeProviderComponent, ConfigUpdater, ConfigUpdaterComponent, DenomGetter,
    DenomGetterComponent, RandomAmountGeneratorComponent, RelayerWallet,
    SetupUpgradeClientTestResultTypeProvider, SetupUpgradeClientTestResultTypeProviderComponent,
    StakingDenom, TransferDenom, UserWallet, ValidatorWallet, WalletGetterComponent,
    WalletsGetterComponent,
};
use hermes_core::test_components::test_case::traits::upgrade_client::{
    SetupUpgradeClientTestHandlerComponent, UpgradeClientHandlerComponent,
};
use hermes_cosmos_core::chain_components::impls::RelayerConfig;
use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainCommandPathGetter, ChainCommandPathGetterComponent,
};
use hermes_cosmos_core::test_components::bootstrap::types::{
    CosmosChainNodeConfig, CosmosGenesisConfig,
};
use hermes_cosmos_core::test_components::chain::impls::{
    CosmosHandleUpgradeClient, SetupCosmosUpgradeClientTest,
};
use hermes_cosmos_core::test_components::chain::types::{CosmosTestWallet, Denom};
use hermes_cosmos_core::test_components::chain_driver::components::CosmosChainDriverComponents as BaseCosmosChainDriverComponents;
use hermes_cosmos_core::test_components::chain_driver::impls::CosmosProposalSetupClientUpgradeResult;
use hermes_cosmos_core::test_components::chain_driver::traits::{
    GrpcPortGetter, GrpcPortGetterComponent, RpcPortGetter, RpcPortGetterComponent,
};
use hermes_cosmos_core::tracing_logging_components::contexts::TracingLogger;
use hermes_cosmos_relayer::contexts::CosmosChain;
use hermes_error::handlers::DebugError;
use hermes_error::impls::UseHermesError;
use hermes_error::types::Error;
use hermes_prelude::*;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use tokio::process::Child;
use toml::to_string_pretty;

/**
   A chain driver for adding test functionalities to a Cosmos chain.
*/
#[cgp_context(CosmosChainDriverComponents)]
#[derive(HasField)]
pub struct CosmosChainDriver {
    pub chain: CosmosChain,
    pub chain_command_path: PathBuf,
    pub chain_processes: Vec<Child>,
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
        UpgradeClientHandlerComponent:
            CosmosHandleUpgradeClient,
        SetupUpgradeClientTestHandlerComponent:
            SetupCosmosUpgradeClientTest,
        LoggerComponent:
            TracingLogger,
        WalletsGetterComponent:
            UseField<symbol!("wallets")>,
        WalletGetterComponent<RelayerWallet>:
            UseField<symbol!("relayer_wallet")>,
        WalletGetterComponent<ValidatorWallet>:
            UseField<symbol!("validator_wallet")>,
        WalletGetterComponent<UserWallet<0>>:
            UseField<symbol!("user_wallet_a")>,
        WalletGetterComponent<UserWallet<1>>:
            UseField<symbol!("user_wallet_b")>,
    }
}

#[cgp_provider(SetupUpgradeClientTestResultTypeProviderComponent)]
impl SetupUpgradeClientTestResultTypeProvider<CosmosChainDriver> for CosmosChainDriverComponents {
    type SetupUpgradeClientTestResult = CosmosProposalSetupClientUpgradeResult;
}

#[cgp_provider(ChainTypeProviderComponent)]
impl ChainTypeProvider<CosmosChainDriver> for CosmosChainDriverComponents {
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

#[cgp_provider(DenomGetterComponent<TransferDenom>)]
impl DenomGetter<CosmosChainDriver, TransferDenom> for CosmosChainDriverComponents {
    fn denom(driver: &CosmosChainDriver, _index: PhantomData<TransferDenom>) -> &Denom {
        &driver.genesis_config.transfer_denom
    }
}

#[cgp_provider(DenomGetterComponent<StakingDenom>)]
impl DenomGetter<CosmosChainDriver, StakingDenom> for CosmosChainDriverComponents {
    fn denom(driver: &CosmosChainDriver, _index: PhantomData<StakingDenom>) -> &Denom {
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
    fn take_chain_process(chain_driver: &mut CosmosChainDriver) -> Vec<Child> {
        core::mem::take(&mut chain_driver.chain_processes)
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

check_components! {
    CanUseCosmosChainDriver for CosmosChainDriver {
        ChainStartupWaiterComponent,
    }
}

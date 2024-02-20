use alloc::collections::BTreeMap;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_cosmos_test_components::chain_driver::components::CosmosChainDriverComponents as BaseCosmosChainDriverComponents;

use std::path::PathBuf;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_cosmos_test_components::chain_driver::traits::grpc_port::GrpcPortGetter;
use hermes_cosmos_test_components::chain_driver::traits::rpc_port::RpcPortGetter;
use hermes_cosmos_test_components::chain_driver::traits::store_wasm_client::WasmClientCodeUploaderComponent;
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::chain_driver::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain_driver::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::ChainHomeDirGetter;
use hermes_test_components::chain_driver::traits::fields::denom_at::{
    DenomGetterAt, StakingDenom, TransferDenom,
};
use hermes_test_components::chain_driver::traits::fields::wallet::{
    RelayerWallet, UserWallet, WalletGetterAt, WalletsGetter,
};
use hermes_test_components::chain_driver::traits::types::chain::{ChainGetter, ProvideChainType};
use hermes_test_components::types::index::Index;
use tokio::process::Child;

/**
   A chain driver for adding test functionalities to a Cosmos chain.
*/
pub struct CosmosChainDriver {
    pub chain: CosmosChain,
    pub chain_command_path: PathBuf,
    pub chain_process: Child,
    pub chain_node_config: CosmosChainNodeConfig,
    pub genesis_config: CosmosGenesisConfig,
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
        ErrorTypeComponent:
            ProvideEyreError,
        ErrorRaiserComponent:
            RaiseDebugError,
        RuntimeTypeComponent:
            ProvideTokioRuntimeType,
        [
            RandomAmountGeneratorComponent,
            EventualAmountAsserterComponent,
            PollAssertDurationGetterComponent,
            WasmClientCodeUploaderComponent,
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

impl ProvideRuntime<CosmosChainDriver> for CosmosChainDriverComponents {
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

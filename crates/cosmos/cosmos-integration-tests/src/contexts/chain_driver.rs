use alloc::collections::BTreeMap;
use hermes_wasm_test_components::components::WasmChainDriverComponents;
use hermes_wasm_test_components::traits::upload_client_code::WasmClientCodeUploaderComponent;
use std::path::PathBuf;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::{DebugError, ProvideCosmosError};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::denom::Denom;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_cosmos_test_components::chain_driver::components::CosmosChainDriverComponents as BaseCosmosChainDriverComponents;
use hermes_cosmos_test_components::chain_driver::traits::deposit_proposal::GovernanceProposalDepositerComponent;
use hermes_cosmos_test_components::chain_driver::traits::grpc_port::GrpcPortGetter;
use hermes_cosmos_test_components::chain_driver::traits::proposal_status::GovernanceProposalStatusQuerierComponent;
use hermes_cosmos_test_components::chain_driver::traits::rpc_port::RpcPortGetter;
use hermes_cosmos_test_components::chain_driver::traits::vote_proposal::GovernanceProposalVoterComponent;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
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
        ErrorTypeComponent: ProvideCosmosError,
        ErrorRaiserComponent: DebugError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            RandomAmountGeneratorComponent,
            GovernanceProposalStatusQuerierComponent,
            GovernanceProposalDepositerComponent,
            GovernanceProposalVoterComponent,
        ]:
            BaseCosmosChainDriverComponents,
        [
            WasmClientCodeUploaderComponent,
        ]:
            WasmChainDriverComponents,
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

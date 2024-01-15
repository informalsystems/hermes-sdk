use alloc::sync::Arc;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::types::chain_config::CosmosChainConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain_driver::impls::address::ProvideStringAddress;
use hermes_cosmos_test_components::chain_driver::impls::amount::ProvideU128AmountWithDenom;
use hermes_cosmos_test_components::chain_driver::impls::chain_id::BuildCosmosChainIdFromString;
use hermes_cosmos_test_components::chain_driver::impls::denom::ProvideIbcDenom;
use hermes_cosmos_test_components::chain_driver::impls::wallet::ProvideCosmosTestWallet;
use hermes_cosmos_test_components::chain_driver::types::denom::Denom;
use hermes_cosmos_test_components::chain_driver::types::wallet::CosmosTestWallet;
use hermes_test_components::chain_driver::traits::build::chain_id::ChainIdFromStringBuilderComponent;
use hermes_test_components::chain_driver::traits::fields::amount::AmountMethodsComponent;
use hermes_test_components::chain_driver::traits::fields::denom_at::DenomGetterAt;
use hermes_test_components::chain_driver::traits::fields::denom_at::StakingDenom;
use hermes_test_components::chain_driver::traits::fields::denom_at::TransferDenom;
use hermes_test_components::chain_driver::traits::fields::wallet::RelayerWallet;
use hermes_test_components::chain_driver::traits::fields::wallet::UserWallet;
use hermes_test_components::chain_driver::traits::fields::wallet::WalletGetterAt;
use hermes_test_components::chain_driver::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain_driver::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain_driver::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain_driver::traits::types::wallet::{
    WalletSignerComponent, WalletTypeComponent,
};
use hermes_test_components::driver::traits::types::chain::ChainGetter;
use hermes_test_components::driver::traits::types::chain::ProvideChainType;
use hermes_test_components::types::index::Index;
use ibc_relayer::config::ChainConfig;
use tokio::process::Child;

/**
   A chain driver for adding test functionalities to a Cosmos chain.
*/
#[derive(Clone)]
pub struct CosmosChainDriver {
    pub base_chain: CosmosChain,
    pub full_node_process: Arc<Child>,
    pub relayer_chain_config: ChainConfig,
    pub chain_config: CosmosChainConfig,
    pub genesis_config: CosmosGenesisConfig,
    pub relayer_wallet: CosmosTestWallet,
    pub user_wallet_a: CosmosTestWallet,
    pub user_wallet_b: CosmosTestWallet,
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
        [
            WalletTypeComponent,
            WalletSignerComponent,
        ]:
            ProvideCosmosTestWallet,
        ChainIdFromStringBuilderComponent:
            BuildCosmosChainIdFromString,
        [
            AmountTypeComponent,
            AmountMethodsComponent,
        ]:
            ProvideU128AmountWithDenom,
        DenomTypeComponent:
            ProvideIbcDenom,
        AddressTypeComponent:
            ProvideStringAddress,
    }
}

impl<Driver> ProvideChainType<Driver> for CosmosChainDriverComponents
where
    Driver: Async,
{
    type Chain = CosmosChain;
}

impl ChainGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain(driver: &CosmosChainDriver) -> &CosmosChain {
        &driver.base_chain
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

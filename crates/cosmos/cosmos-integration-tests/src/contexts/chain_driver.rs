use alloc::collections::BTreeMap;
use std::path::PathBuf;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::transaction::CosmosTxContext;
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain_driver::impls::address::ProvideStringAddress;
use hermes_cosmos_test_components::chain_driver::impls::amount::ProvideU128AmountWithDenom;
use hermes_cosmos_test_components::chain_driver::impls::chain_id::BuildCosmosChainIdFromString;
use hermes_cosmos_test_components::chain_driver::impls::convert_ibc_amout::ConvertCosmosIbcAmount;
use hermes_cosmos_test_components::chain_driver::impls::denom::ProvideIbcDenom;
use hermes_cosmos_test_components::chain_driver::impls::ibc_transfer_timeout::IbcTransferTimeoutAfterSeconds;
use hermes_cosmos_test_components::chain_driver::impls::messages::ibc_transfer::BuildCosmosIbcTransferMessage;
use hermes_cosmos_test_components::chain_driver::impls::query_balance::QueryCosmosBalance;
use hermes_cosmos_test_components::chain_driver::impls::store_wasm_client::UploadWasmClientCodeWithChainCommand;
use hermes_cosmos_test_components::chain_driver::impls::wallet::ProvideCosmosTestWallet;
use hermes_cosmos_test_components::chain_driver::traits::grpc_port::GrpcPortGetter;
use hermes_cosmos_test_components::chain_driver::traits::rpc_port::RpcPortGetter;
use hermes_cosmos_test_components::chain_driver::traits::store_wasm_client::WasmClientCodeUploaderComponent;
use hermes_cosmos_test_components::chain_driver::types::denom::Denom;
use hermes_cosmos_test_components::chain_driver::types::wallet::CosmosTestWallet;
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::chain_driver::impls::default_assert_duration::ProvideDefaultPollAssertDuration;
use hermes_test_components::chain_driver::impls::default_memo::ProvideDefaultMemo;
use hermes_test_components::chain_driver::impls::ibc_transfer::SendIbcTransferMessage;
use hermes_test_components::chain_driver::impls::poll_assert_eventual_amount::PollAssertEventualAmount;
use hermes_test_components::chain_driver::impls::string_memo::ProvideStringMemoType;
use hermes_test_components::chain_driver::traits::assert::eventual_amount::{
    CanAssertEventualAmount, EventualAmountAsserterComponent,
};
use hermes_test_components::chain_driver::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain_driver::traits::build::chain_id::ChainIdFromStringBuilderComponent;
use hermes_test_components::chain_driver::traits::fields::amount::{
    AmountMethodsComponent, IbcTransferredAmountConverterComponent, RandomAmountGeneratorComponent,
};
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::ChainHomeDirGetter;
use hermes_test_components::chain_driver::traits::fields::denom_at::{
    DenomGetterAt, StakingDenom, TransferDenom,
};
use hermes_test_components::chain_driver::traits::fields::memo::DefaultMemoGetterComponent;
use hermes_test_components::chain_driver::traits::fields::timeout::IbcTransferTimeoutCalculatorComponent;
use hermes_test_components::chain_driver::traits::fields::wallet::{
    RelayerWallet, UserWallet, WalletGetterAt, WalletsGetter,
};
use hermes_test_components::chain_driver::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilderComponent;
use hermes_test_components::chain_driver::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain_driver::traits::queries::ibc_transfer::{
    CanIbcTransferToken, TokenIbcTransferrerComponent,
};
use hermes_test_components::chain_driver::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain_driver::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainGetter, HasChainType, ProvideChainType,
};
use hermes_test_components::chain_driver::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain_driver::traits::types::memo::MemoTypeComponent;
use hermes_test_components::chain_driver::traits::types::tx_context::{
    ProvideTxContextType, TxContextGetter,
};
use hermes_test_components::chain_driver::traits::types::wallet::{
    WalletSignerComponent, WalletTypeComponent,
};
use hermes_test_components::types::index::Index;
use tokio::process::Child;

/**
   A chain driver for adding test functionalities to a Cosmos chain.
*/
pub struct CosmosChainDriver {
    pub chain: CosmosChain,
    pub chain_process: Child,
    pub chain_node_config: CosmosChainNodeConfig,
    pub genesis_config: CosmosGenesisConfig,
    pub relayer_wallet: CosmosTestWallet,
    pub user_wallet_a: CosmosTestWallet,
    pub user_wallet_b: CosmosTestWallet,
    pub wallets: BTreeMap<String, CosmosTestWallet>,
}

pub struct CosmosChainDriverComponents;

pub trait UseChainDriver:
    HasChainType<Chain = CosmosChain> + CanIbcTransferToken<CosmosChainDriver> + CanAssertEventualAmount
{
}

impl UseChainDriver for CosmosChainDriver {}

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
            WalletTypeComponent,
            WalletSignerComponent,
        ]:
            ProvideCosmosTestWallet,
        ChainIdFromStringBuilderComponent:
            BuildCosmosChainIdFromString,
        [
            AmountTypeComponent,
            AmountMethodsComponent,
            RandomAmountGeneratorComponent,
        ]:
            ProvideU128AmountWithDenom,
        DenomTypeComponent:
            ProvideIbcDenom,
        AddressTypeComponent:
            ProvideStringAddress,
        MemoTypeComponent:
            ProvideStringMemoType,
        DefaultMemoGetterComponent:
            ProvideDefaultMemo,
        EventualAmountAsserterComponent:
            PollAssertEventualAmount,
        PollAssertDurationGetterComponent:
            ProvideDefaultPollAssertDuration,
        TokenIbcTransferrerComponent:
            SendIbcTransferMessage,
        IbcTransferTimeoutCalculatorComponent:
            IbcTransferTimeoutAfterSeconds<90>,
        IbcTokenTransferMessageBuilderComponent:
            BuildCosmosIbcTransferMessage,
        IbcTransferredAmountConverterComponent:
            ConvertCosmosIbcAmount,
        BalanceQuerierComponent:
            QueryCosmosBalance,
        WasmClientCodeUploaderComponent:
            UploadWasmClientCodeWithChainCommand,
    }
}

impl<Driver> ProvideChainType<Driver> for CosmosChainDriverComponents
where
    Driver: Async,
{
    type Chain = CosmosChain;
}

impl<Driver> ProvideTxContextType<Driver> for CosmosChainDriverComponents
where
    Driver: Async,
{
    type TxContext = CosmosTxContext;
}

impl ChainGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn chain(driver: &CosmosChainDriver) -> &CosmosChain {
        &driver.chain
    }
}

impl TxContextGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn tx_context(driver: &CosmosChainDriver) -> &CosmosTxContext {
        &driver.chain.tx_context
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

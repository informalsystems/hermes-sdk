use core::time::Duration;

use alloc::sync::Arc;
use cgp_core::prelude::*;
use cgp_core::ErrorRaiserComponent;
use cgp_core::ErrorTypeComponent;
use cgp_error_eyre::ProvideEyreError;
use cgp_error_eyre::RaiseDebugError;
use eyre::Error;
use hermes_cosmos_client_components::methods::encode::encode_to_any;
use hermes_cosmos_client_components::traits::grpc_address::HasGrpcAddress;
use hermes_cosmos_client_components::traits::message::DynCosmosMessage;
use hermes_cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::transaction::CosmosTxContext;
use hermes_cosmos_test_components::bootstrap::types::chain_config::CosmosChainConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain_driver::impls::address::ProvideStringAddress;
use hermes_cosmos_test_components::chain_driver::impls::amount::ProvideU128AmountWithDenom;
use hermes_cosmos_test_components::chain_driver::impls::chain_id::BuildCosmosChainIdFromString;
use hermes_cosmos_test_components::chain_driver::impls::denom::ProvideIbcDenom;
use hermes_cosmos_test_components::chain_driver::impls::wallet::ProvideCosmosTestWallet;
use hermes_cosmos_test_components::chain_driver::types::amount::Amount;
use hermes_cosmos_test_components::chain_driver::types::denom::Denom;
use hermes_cosmos_test_components::chain_driver::types::wallet::CosmosTestWallet;
use hermes_relayer_components::runtime::traits::runtime::ProvideRuntime;
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::chain_driver::impls::default_assert_duration::ProvideDefaultPollAssertDuration;
use hermes_test_components::chain_driver::impls::default_memo::ProvideDefaultMemo;
use hermes_test_components::chain_driver::impls::ibc_transfer::SendIbcTransferMessage;
use hermes_test_components::chain_driver::impls::poll_assert_eventual_amount::PollAssertEventualAmount;
use hermes_test_components::chain_driver::impls::string_memo::ProvideStringMemoType;
use hermes_test_components::chain_driver::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain_driver::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain_driver::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain_driver::traits::build::chain_id::ChainIdFromStringBuilderComponent;
use hermes_test_components::chain_driver::traits::fields::amount::AmountMethodsComponent;
use hermes_test_components::chain_driver::traits::fields::amount::IbcTransferredAmountConverter;
use hermes_test_components::chain_driver::traits::fields::amount::RandomAmountGeneratorComponent;
use hermes_test_components::chain_driver::traits::fields::denom_at::DenomGetterAt;
use hermes_test_components::chain_driver::traits::fields::denom_at::StakingDenom;
use hermes_test_components::chain_driver::traits::fields::denom_at::TransferDenom;
use hermes_test_components::chain_driver::traits::fields::memo::DefaultMemoGetterComponent;
use hermes_test_components::chain_driver::traits::fields::timeout::IbcTransferTimeoutCalculator;
use hermes_test_components::chain_driver::traits::fields::wallet::RelayerWallet;
use hermes_test_components::chain_driver::traits::fields::wallet::UserWallet;
use hermes_test_components::chain_driver::traits::fields::wallet::WalletGetterAt;
use hermes_test_components::chain_driver::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilder;
use hermes_test_components::chain_driver::traits::queries::balance::BalanceQuerier;
use hermes_test_components::chain_driver::traits::queries::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain_driver::traits::queries::ibc_transfer::TokenIbcTransferrerComponent;
use hermes_test_components::chain_driver::traits::types::address::AddressTypeComponent;
use hermes_test_components::chain_driver::traits::types::amount::AmountTypeComponent;
use hermes_test_components::chain_driver::traits::types::chain::ChainGetter;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use hermes_test_components::chain_driver::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain_driver::traits::types::memo::MemoTypeComponent;
use hermes_test_components::chain_driver::traits::types::tx_context::ProvideTxContextType;
use hermes_test_components::chain_driver::traits::types::tx_context::TxContextGetter;
use hermes_test_components::chain_driver::traits::types::wallet::{
    WalletSignerComponent, WalletTypeComponent,
};
use hermes_test_components::types::index::Index;
use ibc_proto::cosmos::base::v1beta1::Coin;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::apps::transfer::v1::MsgTransfer;
use ibc_relayer::chain::cosmos::query::balance::query_balance;
use ibc_relayer::config::ChainConfig;
use ibc_relayer_types::core::ics24_host::identifier::ChannelId;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::timestamp::Timestamp;
use ibc_relayer_types::Height;
use prost::EncodeError;
use tokio::process::Child;

use crate::impls::denom::derive_ibc_denom;

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
        &driver.base_chain
    }
}

impl TxContextGetter<CosmosChainDriver> for CosmosChainDriverComponents {
    fn tx_context(driver: &CosmosChainDriver) -> &CosmosTxContext {
        &driver.base_chain.tx_context
    }
}

impl ProvideRuntime<CosmosChainDriver> for CosmosChainDriverComponents {
    fn runtime(chain_driver: &CosmosChainDriver) -> &HermesRuntime {
        &chain_driver.base_chain.runtime
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

impl BalanceQuerier<CosmosChainDriver> for CosmosChainDriverComponents {
    async fn query_balance(
        chain_driver: &CosmosChainDriver,
        address: &String,
        denom: &Denom,
    ) -> Result<Amount, Error> {
        let grpc_address = chain_driver.base_chain.grpc_address();
        let denom_str = denom.to_string();

        let balance = query_balance(grpc_address, address, &denom_str).await?;

        let quantity = balance.amount.parse()?;

        Ok(Amount {
            quantity,
            denom: denom.clone(),
        })
    }
}

impl IbcTransferTimeoutCalculator<CosmosChainDriver> for CosmosChainDriverComponents {
    fn ibc_transfer_timeout_time(
        _chain_driver: &CosmosChainDriver,
        current_time: &Timestamp,
    ) -> Option<Timestamp> {
        let time = (*current_time + Duration::from_secs(90)).unwrap();
        Some(time)
    }

    fn ibc_transfer_timeout_height(
        _chain_driver: &CosmosChainDriver,
        _current_height: &Height,
    ) -> Option<Height> {
        None
    }
}

#[derive(Debug)]
pub struct TokenTransferMessage {
    pub channel_id: ChannelId,
    pub port_id: PortId,
    pub recipient_address: String,
    pub amount: Amount,
    pub memo: Option<String>,
    pub timeout_height: Option<Height>,
    pub timeout_time: Option<Timestamp>,
}

impl DynCosmosMessage for TokenTransferMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Result<Any, EncodeError> {
        let timeout_timestamp = self.timeout_time.unwrap_or_default().nanoseconds();

        let message = MsgTransfer {
            source_port: self.port_id.to_string(),
            source_channel: self.channel_id.to_string(),
            token: Some(Coin {
                denom: self.amount.denom.to_string(),
                amount: self.amount.quantity.to_string(),
            }),
            sender: signer.to_string(),
            receiver: self.recipient_address.clone(),
            timeout_height: self.timeout_height.map(Into::into),
            timeout_timestamp,
            memo: self.memo.clone().unwrap_or_default(),
        };

        encode_to_any("/ibc.applications.transfer.v1.MsgTransfer", &message)
    }
}

impl IbcTokenTransferMessageBuilder<CosmosChainDriver, CosmosChainDriver>
    for CosmosChainDriverComponents
{
    async fn build_ibc_token_transfer_message(
        _chain_driver: &CosmosChainDriver,
        channel_id: &ChannelId,
        port_id: &PortId,
        recipient_address: &String,
        amount: &Amount,
        memo: &Option<String>,
        timeout_height: Option<&Height>,
        timeout_time: Option<&Timestamp>,
    ) -> Result<CosmosMessage, Error> {
        let message = TokenTransferMessage {
            channel_id: channel_id.clone(),
            port_id: port_id.clone(),
            recipient_address: recipient_address.clone(),
            amount: amount.clone(),
            memo: memo.clone(),
            timeout_height: timeout_height.cloned(),
            timeout_time: timeout_time.cloned(),
        };

        Ok(message.to_cosmos_message())
    }
}

impl IbcTransferredAmountConverter<CosmosChainDriver, CosmosChainDriver>
    for CosmosChainDriverComponents
{
    fn ibc_transfer_amount_from(
        counterparty_amount: &Amount,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<Amount, Error> {
        let denom = derive_ibc_denom(port_id, channel_id, &counterparty_amount.denom)?;

        Ok(Amount {
            quantity: counterparty_amount.quantity,
            denom,
        })
    }

    fn transmute_counterparty_amount(counterparty_amount: &Amount, denom: &Denom) -> Amount {
        Amount {
            quantity: counterparty_amount.quantity,
            denom: denom.clone(),
        }
    }
}

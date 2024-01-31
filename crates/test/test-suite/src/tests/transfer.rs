use alloc::format;

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::logger::traits::log::CanLog;
use hermes_test_components::chain_driver::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain_driver::traits::fields::amount::{
    CanConvertIbcTransferredAmount, CanGenerateRandomAmount, HasAmountMethods,
};
use hermes_test_components::chain_driver::traits::fields::denom_at::{HasDenomAt, TransferDenom};
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWalletAt, UserWallet};
use hermes_test_components::chain_driver::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain_driver::traits::queries::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::driver::traits::channel_at::HasChannelAt;
use hermes_test_components::driver::traits::types::chain_at::HasChainTypeAt;
use hermes_test_components::driver::traits::types::chain_driver_at::HasChainDriverAt;
use hermes_test_components::driver::traits::types::relay_driver_at::HasRelayDriverAt;
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::test_case::traits::test_case::TestCase;
use hermes_test_components::types::index::{Index, Twindex};

pub struct TestIbcTransfer;

#[async_trait]
impl<Driver, ChainA, ChainB, ChainDriverA, ChainDriverB, RelayDriver> TestCase<Driver>
    for TestIbcTransfer
where
    Driver: HasErrorType
        + HasChainTypeAt<0, Chain = ChainA>
        + HasChainTypeAt<1, Chain = ChainB>
        + HasChainDriverAt<0, ChainDriver = ChainDriverA>
        + HasChainDriverAt<1, ChainDriver = ChainDriverB>
        + HasRelayDriverAt<0, 1, RelayDriver = RelayDriver>
        + HasChannelAt<0, 1>
        + HasChannelAt<1, 0>
        + CanLog,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasDenomAt<TransferDenom, 0>
        + CanQueryBalance
        + HasWalletAt<UserWallet, 0>
        + HasWalletAt<UserWallet, 1>
        + HasAmountMethods
        + CanGenerateRandomAmount
        + CanAssertEventualAmount
        + CanIbcTransferToken<ChainDriverB>
        + CanConvertIbcTransferredAmount<ChainDriverB>,
    ChainDriverB: HasChain<Chain = ChainB>
        + HasWalletAt<UserWallet, 0>
        + HasAmountMethods
        + CanQueryBalance
        + CanGenerateRandomAmount
        + CanAssertEventualAmount
        + CanIbcTransferToken<ChainDriverA>
        + CanConvertIbcTransferredAmount<ChainDriverA>,
    RelayDriver: CanRunRelayerInBackground,
    ChainA: HasIbcChainTypes<ChainB> + HasChainId + HasIbcPacketTypes<ChainB>,
    ChainB: HasIbcChainTypes<ChainA> + HasChainId + HasIbcPacketTypes<ChainA>,
    Driver::Error: From<ChainDriverA::Error> + From<ChainDriverB::Error> + From<RelayDriver::Error>,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_at(Index::<0>);

        let chain_driver_b = driver.chain_driver_at(Index::<1>);

        let relay_driver = driver.relay_driver_at(Twindex::<0, 1>);

        let chain_a = chain_driver_a.chain();

        let chain_id_a = chain_a.chain_id();

        let chain_b = chain_driver_b.chain();

        let chain_id_b = chain_b.chain_id();

        let wallet_a1 = chain_driver_a.wallet_at(UserWallet, Index::<0>);

        let address_a1 = ChainDriverA::wallet_address(wallet_a1);

        let wallet_b = chain_driver_b.wallet_at(UserWallet, Index::<0>);

        let address_b = ChainDriverB::wallet_address(wallet_b);

        let denom_a = chain_driver_a.denom_at(TransferDenom, Index::<0>);

        let balance_a1 = chain_driver_a.query_balance(address_a1, denom_a).await?;

        let a_to_b_amount = chain_driver_a.random_amount(1000, &balance_a1).await;

        let channel_id_a = driver.channel_id_at(Twindex::<0, 1>);

        let port_id_a = driver.port_id_at(Twindex::<0, 1>);

        let channel_id_b = driver.channel_id_at(Twindex::<1, 0>);

        let port_id_b = driver.port_id_at(Twindex::<1, 0>);

        let _relayer = relay_driver.run_relayer_in_background().await?;

        driver.log_info(&format!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chain_id_a, chain_id_b, a_to_b_amount, denom_a
        ));

        chain_driver_a
            .ibc_transfer_token(
                channel_id_a,
                port_id_a,
                wallet_a1,
                address_b,
                &a_to_b_amount,
            )
            .await?;

        let balance_a2 = ChainDriverA::subtract_amount(&balance_a1, &a_to_b_amount)?;

        let balance_a3 = chain_driver_a.query_balance(address_a1, denom_a).await?;

        assert_eq!(balance_a2, balance_a3);

        let balance_b1 =
            ChainDriverB::ibc_transfer_amount_from(&a_to_b_amount, channel_id_b, port_id_b)?;

        driver.log_info(&format!(
            "Waiting for user on chain B to receive IBC transferred amount of {}",
            balance_b1
        ));

        chain_driver_b
            .assert_eventual_amount(address_b, &balance_b1)
            .await?;

        let wallet_a2 = chain_driver_a.wallet_at(UserWallet, Index::<1>);

        let address_a2 = ChainDriverA::wallet_address(wallet_a2);

        let b_to_a_amount = chain_driver_b.random_amount(500, &balance_b1).await;

        driver.log_info(&format!(
            "Sending IBC transfer from chain {} to chain {} with amount of {}",
            chain_id_b, chain_id_a, b_to_a_amount,
        ));

        chain_driver_b
            .ibc_transfer_token(
                channel_id_b,
                port_id_b,
                wallet_b,
                address_a2,
                &b_to_a_amount,
            )
            .await?;

        let balance_b2 = ChainDriverB::subtract_amount(&balance_b1, &b_to_a_amount)?;

        let denom_b = ChainDriverB::amount_denom(&balance_b1);

        let balance_b3 = chain_driver_b.query_balance(address_b, denom_b).await?;

        assert_eq!(balance_b2, balance_b3);

        let balance_a4 = chain_driver_a.query_balance(address_a2, denom_a).await?;

        let balance_a5 = ChainDriverA::add_amount(
            &balance_a4,
            &ChainDriverA::transmute_counterparty_amount(&b_to_a_amount, denom_a),
        )?;

        chain_driver_a
            .assert_eventual_amount(address_a2, &balance_a5)
            .await?;

        driver.log_info(&format!(
            "successfully performed reverse IBC transfer from chain {} back to chain {}",
            chain_id_b, chain_id_a,
        ));

        Ok(())
    }
}

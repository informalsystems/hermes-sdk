use alloc::format;

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::logger::traits::log::CanLog;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::fields::amount::{
    CanConvertIbcTransferredAmount, CanGenerateRandomAmount, HasAmountMethods,
};
use hermes_test_components::chain::traits::fields::channel_at::HasChannelAt;
use hermes_test_components::chain::traits::fields::denom_at::HasDenomAt;
use hermes_test_components::chain::traits::fields::wallet::{HasOneUserWallet, HasTwoUserWallets};
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::queries::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::driver::traits::background_relayer::HasBackgroundRelayer;
use hermes_test_components::driver::traits::types::chain_at::{
    HasChainAt, HasOneChain, HasTwoChains,
};
use hermes_test_components::test_case::traits::test_case::TestCase;

pub struct TestIbcTransfer;

#[async_trait]
impl<Driver, ChainA, ChainB> TestCase<Driver> for TestIbcTransfer
where
    Driver: HasErrorType
        + HasChainAt<0, Chain = ChainA>
        + HasChainAt<1, Chain = ChainB>
        + CanLog
        + HasBackgroundRelayer,
    ChainA: HasIbcChainTypes<ChainB>
        + HasChannelAt<ChainB, 0>
        + HasDenomAt<0>
        + HasChainId
        + HasTwoUserWallets
        + HasAmountMethods
        + CanQueryBalance
        + CanGenerateRandomAmount
        + CanAssertEventualAmount
        + CanIbcTransferToken<ChainB>
        + CanConvertIbcTransferredAmount<ChainB>,
    ChainB: HasIbcChainTypes<ChainA>
        + HasChannelAt<ChainA, 0>
        + HasChainId
        + HasOneUserWallet
        + HasAmountMethods
        + CanQueryBalance
        + CanGenerateRandomAmount
        + CanAssertEventualAmount
        + CanIbcTransferToken<ChainA>
        + CanConvertIbcTransferredAmount<ChainA>,
    Driver::Error: From<ChainA::Error> + From<ChainB::Error>,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let chain_a = driver.first_chain();

        let chain_id_a = chain_a.chain_id();

        let chain_b = driver.second_chain();

        let chain_id_b = chain_b.chain_id();

        let wallet_a1 = chain_a.first_user_wallet();

        let address_a1 = ChainA::wallet_address(wallet_a1);

        let wallet_b = chain_b.first_user_wallet();

        let address_b = ChainB::wallet_address(wallet_b);

        let denom_a = chain_a.denom();

        let balance_a1 = chain_a.query_balance(address_a1, denom_a).await?;

        let a_to_b_amount = ChainA::random_amount(1000, &balance_a1);

        let channel_id_a = chain_a.channel_id();

        let port_id_a = chain_a.port_id();

        let channel_id_b = chain_b.channel_id();

        let port_id_b = chain_b.port_id();

        driver.start_relayer_in_background();

        driver.log_info(&format!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chain_id_a, chain_id_b, a_to_b_amount, denom_a
        ));

        chain_a
            .ibc_transfer_token(
                channel_id_a,
                port_id_a,
                wallet_a1,
                address_b,
                &a_to_b_amount,
            )
            .await?;

        let balance_a2 = ChainA::subtract_amount(&balance_a1, &a_to_b_amount)?;

        let balance_a3 = chain_a.query_balance(address_a1, denom_a).await?;

        assert_eq!(balance_a2, balance_a3);

        let balance_b1 = ChainB::ibc_transfer_amount_from(&a_to_b_amount, channel_id_b, port_id_b);

        driver.log_info(&format!(
            "Waiting for user on chain B to receive IBC transferred amount of {}",
            balance_b1
        ));

        chain_b
            .assert_eventual_amount(address_b, &balance_b1)
            .await?;

        let wallet_a2 = chain_a.second_user_wallet();

        let address_a2 = ChainA::wallet_address(wallet_a2);

        let b_to_a_amount = ChainB::random_amount(500, &balance_b1);

        driver.log_info(&format!(
            "Sending IBC transfer from chain {} to chain {} with amount of {}",
            chain_id_b, chain_id_a, b_to_a_amount,
        ));

        chain_b
            .ibc_transfer_token(
                channel_id_b,
                port_id_b,
                wallet_b,
                address_a2,
                &b_to_a_amount,
            )
            .await?;

        let balance_b2 = ChainB::subtract_amount(&balance_b1, &b_to_a_amount)?;

        let denom_b = ChainB::amount_denom(&balance_b1);

        let balance_b3 = chain_b.query_balance(address_b, denom_b).await?;

        assert_eq!(balance_b2, balance_b3);

        let balance_a4 = chain_a.query_balance(address_a2, denom_a).await?;

        let balance_a5 = ChainA::add_amount(
            &balance_a4,
            &ChainA::transmute_counterparty_amount(&b_to_a_amount, denom_a),
        )?;

        chain_a
            .assert_eventual_amount(address_a2, &balance_a5)
            .await?;

        driver.log_info(&format!(
            "successfully performed reverse IBC transfer from chain {} back to chain {}",
            chain_id_b, chain_id_a,
        ));

        Ok(())
    }
}

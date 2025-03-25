use alloc::format;
use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_logging_components::traits::logger::CanLogMessage;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::amount::CanConvertIbcTransferredAmount;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain::traits::types::amount::{HasAmountMethods, HasAmountType};
use hermes_test_components::chain::traits::types::memo::HasDefaultMemo;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::amount::CanGenerateRandomAmount;
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWallet, UserWallet};
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestIbcTransfer<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestIbcTransfer<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestIbcTransfer<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let logger = driver.logger();

        let chain_driver_a = driver.chain_driver_a();

        let chain_driver_b = driver.chain_driver_b();

        let relay_driver = driver.relay_driver();

        let chain_a = driver.chain_a();

        let chain_id_a = chain_a.chain_id();

        let chain_b = driver.chain_b();

        let chain_id_b = chain_b.chain_id();

        let wallet_a1 = driver.user_wallet_a1();

        let address_a1 = Driver::ChainA::wallet_address(wallet_a1);

        let wallet_b = driver.user_wallet_b1();

        let address_b = Driver::ChainB::wallet_address(wallet_b);

        let denom_a = driver.transfer_denom_a();

        let balance_a1 = chain_a
            .query_balance(address_a1, denom_a)
            .await
            .map_err(Driver::raise_error)?;

        let a_to_b_amount = chain_driver_a.random_amount(1000, &balance_a1).await;

        let channel_id_a = driver.channel_id_a();

        let port_id_a = driver.port_id_a();

        let channel_id_b = driver.channel_id_b();

        let port_id_b = driver.port_id_b();

        let _relayer = relay_driver
            .run_relayer_in_background()
            .await
            .map_err(Driver::raise_error)?;

        logger
            .log_message(&format!(
                "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
                chain_id_a, chain_id_b, a_to_b_amount, denom_a
            ))
            .await;

        let balance_b1 = chain_b
            .ibc_transfer_amount_from(PhantomData, &a_to_b_amount, channel_id_b, port_id_b)
            .await
            .map_err(Driver::raise_error)?;

        chain_a
            .ibc_transfer_token(
                PhantomData,
                channel_id_a,
                port_id_a,
                wallet_a1,
                address_b,
                &a_to_b_amount,
                &chain_a.default_memo(),
                &chain_b
                    .query_chain_status()
                    .await
                    .map_err(Driver::raise_error)?,
            )
            .await
            .map_err(Driver::raise_error)?;

        let balance_a2 = Driver::ChainA::subtract_amount(&balance_a1, &a_to_b_amount)
            .map_err(Driver::raise_error)?;

        let balance_a3 = chain_a
            .query_balance(address_a1, denom_a)
            .await
            .map_err(Driver::raise_error)?;

        assert_eq!(balance_a2, balance_a3);

        logger
            .log_message(&format!(
                "Waiting for user on chain B to receive IBC transferred amount of {}",
                balance_b1
            ))
            .await;

        chain_b
            .assert_eventual_amount(address_b, &balance_b1)
            .await
            .map_err(Driver::raise_error)?;

        let wallet_a2 = chain_driver_a.wallet(PhantomData::<UserWallet<1>>);

        let address_a2 = Driver::ChainA::wallet_address(wallet_a2);

        let b_to_a_amount = chain_driver_b.random_amount(500, &balance_b1).await;

        logger
            .log_message(&format!(
                "Sending IBC transfer from chain {} to chain {} with amount of {}",
                chain_id_b, chain_id_a, b_to_a_amount,
            ))
            .await;

        chain_b
            .ibc_transfer_token(
                PhantomData,
                channel_id_b,
                port_id_b,
                wallet_b,
                address_a2,
                &b_to_a_amount,
                &chain_b.default_memo(),
                &chain_a
                    .query_chain_status()
                    .await
                    .map_err(Driver::raise_error)?,
            )
            .await
            .map_err(Driver::raise_error)?;

        let balance_b2 = Driver::ChainB::subtract_amount(&balance_b1, &b_to_a_amount)
            .map_err(Driver::raise_error)?;

        let denom_b = Driver::ChainB::amount_denom(&balance_b1);

        let balance_b3 = chain_b
            .query_balance(address_b, denom_b)
            .await
            .map_err(Driver::raise_error)?;

        assert_eq!(balance_b2, balance_b3);

        let balance_a4 = chain_a
            .query_balance(address_a2, denom_a)
            .await
            .map_err(Driver::raise_error)?;

        let balance_a5 = Driver::ChainA::add_amount(
            &balance_a4,
            &chain_a
                .transmute_counterparty_amount(PhantomData, &b_to_a_amount, denom_a)
                .await
                .map_err(Driver::raise_error)?,
        )
        .map_err(Driver::raise_error)?;

        chain_a
            .assert_eventual_amount(address_a2, &balance_a5)
            .await
            .map_err(Driver::raise_error)?;

        logger
            .log_message(&format!(
                "successfully performed reverse IBC transfer from chain {} back to chain {}",
                chain_id_b, chain_id_a,
            ))
            .await;

        Ok(())
    }
}

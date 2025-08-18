use alloc::format;
use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::Index;
use hermes_chain_components::traits::CanQueryConsensusStateHeights;
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasChainId;
use hermes_test_components::chain::traits::{
    CanAssertEventualAmount, CanCliTransferToken, CanConvertIbcTransferredAmount, CanQueryBalance,
    HasAmountMethods, HasWalletType,
};
use hermes_test_components::chain_driver::traits::CanGenerateRandomAmount;
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::alloc::string::ToString;
use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestBatchIbcTransfer<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestBatchIbcTransfer<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestBatchIbcTransfer<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_a();

        let relay_driver = driver.relay_driver();

        let chain_a = driver.chain_a();

        let chain_id_a = chain_a.chain_id();

        let chain_b = driver.chain_b();

        let chain_id_b = chain_b.chain_id();

        let wallet_a = driver.user_wallet_a1();

        let sender_address = Driver::ChainA::wallet_address(wallet_a);

        let wallet_b = driver.user_wallet_b1();

        let recipient_address = Driver::ChainB::wallet_address(wallet_b);

        let denom_a = driver.transfer_denom_a();

        let stake_denom_a = driver.staking_denom_a();

        let balance_a1 = chain_a
            .query_balance(sender_address, denom_a)
            .await
            .map_err(Driver::raise_error)?;

        let a_to_b_amount = chain_driver_a.fixed_amount(1234, denom_a).await;

        let number_of_transfers = 8;

        let expected_final_amount_a_to_b = chain_driver_a
            .fixed_amount(1234 * number_of_transfers, denom_a)
            .await;

        let fee_amount = chain_driver_a.fixed_amount(300000, stake_denom_a).await;

        let channel_id_a = driver.channel_id_a();

        let port_id_a = driver.port_id_a();

        let channel_id_b = driver.channel_id_b();

        let port_id_b = driver.port_id_b();

        let client_id_a = driver.client_id_a();

        let client_id_b = driver.client_id_b();

        let _handle = relay_driver
            .run_relayer_in_background()
            .await
            .map_err(Driver::raise_error)?;

        driver
            .log_message(&format!(
                "Sending {number_of_transfers} IBC transfers from chain {chain_id_a} to chain {chain_id_b} \
                with amount of {a_to_b_amount} {denom_a}"
            ))
            .await;

        let expected_balance_b = chain_b
            .ibc_transfer_amount_from(
                PhantomData,
                &expected_final_amount_a_to_b,
                channel_id_b,
                port_id_b,
            )
            .await
            .map_err(Driver::raise_error)?;

        let consensus_heights_a_before_txs = chain_a
            .query_consensus_state_heights(client_id_a)
            .await
            .map_err(Driver::raise_error)?;
        let consensus_heights_b_before_txs = chain_b
            .query_consensus_state_heights(client_id_b)
            .await
            .map_err(Driver::raise_error)?;

        // Create 10 transactions which should be batched together with a maximum of 2
        // client updates since the `BatchConfig` `max_delay` is configured to 10 seconds
        // for tests
        for _ in 0..number_of_transfers {
            chain_driver_a
                .cli_transfer_token(
                    port_id_a.to_string().as_str(),
                    channel_id_a.to_string().as_str(),
                    sender_address.to_string().as_str(),
                    recipient_address.to_string().as_str(),
                    a_to_b_amount.to_string().as_str(),
                    fee_amount.to_string().as_str(),
                )
                .await
                .map_err(Driver::raise_error)?;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        tokio::time::sleep(Duration::from_secs(3)).await;

        let expected_balance_a =
            Driver::ChainA::subtract_amount(&balance_a1, &expected_final_amount_a_to_b)
                .map_err(Driver::raise_error)?;

        let queried_balance_a = chain_a
            .query_balance(sender_address, denom_a)
            .await
            .map_err(Driver::raise_error)?;

        assert_eq!(expected_balance_a, queried_balance_a);

        driver
            .log_message(&format!(
                "Waiting for user on chain B to receive IBC transferred amount of {expected_balance_b}"
            ))
            .await;

        chain_b
            .assert_eventual_amount(recipient_address, &expected_balance_b)
            .await
            .map_err(Driver::raise_error)?;

        tokio::time::sleep(Duration::from_secs(1)).await;

        let consensus_heights_a_after_txs = chain_a
            .query_consensus_state_heights(client_id_a)
            .await
            .map_err(Driver::raise_error)?;
        let consensus_heights_b_after_txs = chain_b
            .query_consensus_state_heights(client_id_b)
            .await
            .map_err(Driver::raise_error)?;

        assert!(
            consensus_heights_a_before_txs.len() < consensus_heights_a_after_txs.len() + 3,
            "Expected at most 2 client updates for client A, but got {}: {:?}",
            consensus_heights_a_after_txs.len() - consensus_heights_a_before_txs.len(),
            consensus_heights_a_after_txs
        );
        assert!(
            consensus_heights_b_before_txs.len() < consensus_heights_b_after_txs.len() + 3,
            "Expected at most 2 client updates for client B, but got {}: {:?}",
            consensus_heights_a_after_txs.len() - consensus_heights_a_before_txs.len(),
            consensus_heights_a_after_txs
        );

        Ok(())
    }
}

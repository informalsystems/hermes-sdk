use alloc::{format, vec};
use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::Index;
use hermes_chain_components::traits::{
    CanQueryChainStatus, CanQueryConsensusStateHeights, CanQueryPacketIsCleared,
    CanQueryPacketIsReceived, HasPacketSequence,
};
use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasChainId;
use hermes_test_components::chain::traits::{
    CanAssertEventualAmount, CanConvertIbcTransferredAmount, CanIbcTransferToken, CanQueryBalance,
    HasAmountMethods, HasDefaultMemo, HasWalletType,
};
use hermes_test_components::chain_driver::traits::{CanGenerateRandomAmount, HasChain};
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestBatchIbcTransfer<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestBatchIbcTransfer<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestBatchIbcTransfer<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B> + CanRaiseAsyncError<&'static str>,
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

        let balance_a1 = chain_a
            .query_balance(sender_address, denom_a)
            .await
            .map_err(Driver::raise_error)?;

        let a_to_b_amount = chain_driver_a.fixed_amount(1234, denom_a).await;

        let number_of_transfers = 50;

        let expected_final_amount_a_to_b = chain_driver_a
            .fixed_amount(1234 * number_of_transfers, denom_a)
            .await;

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

        let mut packet_sequences = vec![];
        for _ in 0..number_of_transfers {
            let packet = chain_a
                .ibc_transfer_token(
                    PhantomData,
                    channel_id_a,
                    port_id_a,
                    wallet_a,
                    recipient_address,
                    &a_to_b_amount,
                    &chain_a.default_memo(),
                    &chain_b
                        .query_chain_status()
                        .await
                        .map_err(Driver::raise_error)?,
                )
                .await
                .map_err(Driver::raise_error)?;

            let sequence = Driver::ChainA::packet_sequence(&packet);

            packet_sequences.push(sequence);

            tokio::time::sleep(Duration::from_millis(500)).await;
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

        let client_a_consensus_height_diff =
            consensus_heights_a_after_txs.len() - consensus_heights_a_before_txs.len();
        let client_b_consensus_height_diff =
            consensus_heights_b_after_txs.len() - consensus_heights_b_before_txs.len();

        // Batching is done every 30 seconds and we create a transfer every second.
        // Due to delays when creating transfers we might have more updates, thus take a margin of 2 when
        // asserting the value
        let expected_max_updates = (number_of_transfers as f64 / 30.0).ceil() as usize + 2;

        assert!(
            client_a_consensus_height_diff <= expected_max_updates,
            "Expected at most {expected_max_updates} client updates for client A, but got {client_a_consensus_height_diff}");
        assert!(
            client_b_consensus_height_diff <= expected_max_updates,
            "Expected at most {expected_max_updates} client updates for client B, but got {client_b_consensus_height_diff}");

        driver
            .log_message(&format!(
                "Batching test was successful. Client A was updated: {client_a_consensus_height_diff} times and Client B {client_b_consensus_height_diff} times"
            ))
            .await;

        tokio::time::sleep(Duration::from_secs(3)).await;

        for sequence in packet_sequences.iter() {
            driver
                .log_message(&format!(
                    "Will assert packet with sequence `{sequence:?}` has been cleared"
                ))
                .await;

            let is_received = chain_b
                .query_packet_is_received(port_id_b, channel_id_b, sequence)
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_received);

            let mut cleared_acks = false;

            for _ in 0..100 {
                let is_cleared = chain_driver_a
                    .chain()
                    .query_packet_is_cleared(port_id_a, channel_id_a, sequence)
                    .await
                    .map_err(Driver::raise_error)?;

                if is_cleared {
                    cleared_acks = true;
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            if !cleared_acks {
                return Err(Driver::raise_error(
                    "Batch transactions test failed to properly clear all transfers",
                ));
            }
        }
        Ok(())
    }
}

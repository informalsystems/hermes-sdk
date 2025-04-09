use alloc::format;
use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::packet::fields::HasPacketSequence;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::packet_is_cleared::CanQueryPacketIsCleared;
use hermes_relayer_components::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::relay::traits::auto_relayer::CanAutoRelayWithHeights;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain::traits::types::amount::HasAmountMethods;
use hermes_test_components::chain::traits::types::memo::HasDefaultMemo;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::amount::CanGenerateRandomAmount;
use hermes_test_components::chain_driver::traits::fields::denom::{HasDenom, TransferDenom};
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWallet, UserWallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestPacketClearing<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestPacketClearing<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestPacketClearing<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_a();

        let chain_driver_b = driver.chain_driver_b();

        let relay_a_to_b = driver.relay_a_to_b();

        let chain_a = driver.chain_a();

        let chain_id_a = chain_a.chain_id();

        let chain_b = driver.chain_b();

        let chain_id_b = chain_b.chain_id();

        let channel_id_a = driver.channel_id_a();

        let port_id_a = driver.port_id_a();

        let channel_id_b = driver.channel_id_b();

        let port_id_b = driver.port_id_b();

        driver
            .log_message(&format!(
                "Test clearing pending recv from chain `{chain_id_a}` to chain `{chain_id_b}` and pending ack from `{chain_id_b}` to `{chain_id_a}`"
            ))
            .await;

        let sender_wallet = chain_driver_a.wallet(PhantomData::<UserWallet>);

        let sender_address = Driver::ChainA::wallet_address(sender_wallet);

        let receiver_wallet = chain_driver_b.wallet(PhantomData::<UserWallet>);

        let receiver_address = Driver::ChainB::wallet_address(receiver_wallet);

        let denom = chain_driver_a.denom(PhantomData::<TransferDenom>);

        let start_height_a = chain_a
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let start_height_b = chain_b
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let initial_balance_sender = chain_a
            .query_balance(sender_address, denom)
            .await
            .map_err(Driver::raise_error)?;

        let transfer_amount_1 = chain_driver_a
            .random_amount(1000, &initial_balance_sender)
            .await;

        let packet_1 = chain_a
            .ibc_transfer_token(
                PhantomData,
                channel_id_a,
                port_id_a,
                sender_wallet,
                receiver_address,
                &transfer_amount_1,
                &chain_a.default_memo(),
                &chain_b
                    .query_chain_status()
                    .await
                    .map_err(Driver::raise_error)?,
            )
            .await
            .map_err(Driver::raise_error)?;

        let sequence_1 = Driver::ChainA::packet_sequence(&packet_1);

        // Relay RecvPacket only so that only the ack is not relayed
        {
            let src_chain_height = chain_a
                .query_chain_height()
                .await
                .map_err(Driver::raise_error)?;

            relay_a_to_b
                .relay_receive_packet(&src_chain_height, &packet_1)
                .await
                .map_err(Driver::raise_error)?;
        }

        let expected_sender_balance =
            Driver::ChainA::subtract_amount(&initial_balance_sender, &transfer_amount_1)
                .map_err(Driver::raise_error)?;

        let current_sender_balance = chain_a
            .query_balance(sender_address, denom)
            .await
            .map_err(Driver::raise_error)?;

        assert_eq!(expected_sender_balance, current_sender_balance);

        // Assert only the ack is pending for packet with sequence 1
        {
            let is_received = chain_b
                .query_packet_is_received(port_id_b, channel_id_b, &sequence_1)
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_received);

            let is_cleared = chain_a
                .query_packet_is_cleared(port_id_a, channel_id_a, &sequence_1)
                .await
                .map_err(Driver::raise_error)?;

            assert!(!is_cleared);
        }

        let transfer_amount_2 = chain_driver_a
            .random_amount(1000, &current_sender_balance)
            .await;

        let packet_2 = chain_a
            .ibc_transfer_token(
                PhantomData,
                channel_id_a,
                port_id_a,
                sender_wallet,
                receiver_address,
                &transfer_amount_2,
                &chain_a.default_memo(),
                &chain_b
                    .query_chain_status()
                    .await
                    .map_err(Driver::raise_error)?,
            )
            .await
            .map_err(Driver::raise_error)?;

        let sequence_2 = Driver::ChainA::packet_sequence(&packet_2);

        {
            let expected_balance_sender =
                Driver::ChainA::subtract_amount(&current_sender_balance, &transfer_amount_2)
                    .map_err(Driver::raise_error)?;

            let current_sender_balance = chain_a
                .query_balance(sender_address, denom)
                .await
                .map_err(Driver::raise_error)?;

            assert_eq!(expected_balance_sender, current_sender_balance);
        }

        // Assert both recv and ack are pending for packet with sequence 2
        {
            let is_received = chain_b
                .query_packet_is_received(port_id_b, channel_id_b, &sequence_2)
                .await
                .map_err(Driver::raise_error)?;

            assert!(!is_received);

            let is_cleared = chain_a
                .query_packet_is_cleared(port_id_a, channel_id_a, &sequence_2)
                .await
                .map_err(Driver::raise_error)?;

            assert!(!is_cleared);
        }

        let end_height_a = chain_a
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let end_height_b = chain_b
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        // Perform clearing on the start-end height ranges

        relay_a_to_b
            .auto_relay_with_heights(SourceTarget, &start_height_a, Some(&end_height_a))
            .await
            .map_err(Driver::raise_error)?;

        relay_a_to_b
            .auto_relay_with_heights(DestinationTarget, &start_height_b, Some(&end_height_b))
            .await
            .map_err(Driver::raise_error)?;

        // Assert both recv and ack have been cleared for packet with sequence 1 and 2

        {
            let is_received = chain_b
                .query_packet_is_received(port_id_b, channel_id_b, &sequence_1)
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_received);
        }

        {
            let is_received = chain_b
                .query_packet_is_received(port_id_b, channel_id_b, &sequence_2)
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_received);
        }

        {
            let is_cleared = chain_driver_a
                .chain()
                .query_packet_is_cleared(port_id_a, channel_id_a, &sequence_1)
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_cleared);
        }

        {
            let is_cleared = chain_driver_a
                .chain()
                .query_packet_is_cleared(port_id_a, channel_id_a, &sequence_2)
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_cleared);
        }

        driver
            .log_message(&format!(
                "successfully performed packet clearing between chain `{chain_id_a}` and chain `{chain_id_b}`"
            ))
            .await;

        Ok(())
    }
}

use alloc::vec::Vec;
use alloc::{format, vec};
use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_logging_components::traits::logger::CanLogMessage;
use hermes_relayer_components::birelay::traits::{CanAutoBiRelay, HasTwoWayRelay};
use hermes_relayer_components::chain::traits::packet::fields::HasPacketSequence;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::packet_is_cleared::CanQueryPacketIsCleared;
use hermes_relayer_components::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayAt;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
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
        let logger = driver.logger();

        let chain_driver_a = driver.chain_driver_at(PhantomData::<A>);

        let chain_driver_b = driver.chain_driver_at(PhantomData::<B>);

        let relay_driver = driver.relay_driver_at(PhantomData::<(A, B)>);

        let birelay = relay_driver.birelay_at(PhantomData);

        let relay_a_to_b = birelay.relay_a_to_b();

        let relay_b_to_a = birelay.relay_b_to_a();

        let chain_a = chain_driver_a.chain();

        let chain_id_a = chain_a.chain_id();

        let chain_b = chain_driver_b.chain();

        let chain_id_b = chain_b.chain_id();

        let channel_id_a = driver.channel_id_at(PhantomData::<(A, B)>);

        let port_id_a = driver.port_id_at(PhantomData::<(A, B)>);

        let channel_id_b = driver.channel_id_at(PhantomData::<(B, A)>);

        let port_id_b = driver.port_id_at(PhantomData::<(B, A)>);

        logger
            .log_message(&format!(
                "Test clearing pending recv from chain `{chain_id_a}` to chain `{chain_id_b}` and pending ack from `{chain_id_b}` to `{chain_id_a}`"
            ))
            .await;

        let a_to_b_sequences = run_one_way_clearing_test::<
            Driver,
            Driver::RelayAToB,
            Driver::ChainA,
            Driver::ChainDriverA,
            Driver::ChainB,
            Driver::ChainDriverB,
        >(
            chain_driver_a,
            chain_driver_b,
            relay_a_to_b,
            channel_id_a,
            port_id_a,
            channel_id_b,
            port_id_b,
        )
        .await?;

        logger
            .log_message(&format!(
                "Test clearing pending recv from chain `{chain_id_b}` to chain `{chain_id_a}` and pending ack from `{chain_id_a}` to `{chain_id_b}`"
            ))
            .await;

        let b_to_a_sequences = run_one_way_clearing_test::<
            Driver,
            Driver::RelayBToA,
            Driver::ChainB,
            Driver::ChainDriverB,
            Driver::ChainA,
            Driver::ChainDriverA,
        >(
            chain_driver_b,
            chain_driver_a,
            relay_b_to_a,
            channel_id_b,
            port_id_b,
            channel_id_a,
            port_id_a,
        )
        .await?;

        birelay
            .auto_bi_relay(Some(Duration::from_secs(20)), Some(Duration::from_secs(0)))
            .await
            .unwrap();

        // Assert both recv and ack have been cleared for packet with sequence 1 and 2
        {
            let is_received = chain_driver_a
                .chain()
                .query_packet_is_received(port_id_a, channel_id_a, &b_to_a_sequences[1])
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_received);

            let is_received = chain_driver_b
                .chain()
                .query_packet_is_received(port_id_b, channel_id_b, &a_to_b_sequences[1])
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_received);

            birelay
                .auto_bi_relay(Some(Duration::from_secs(15)), Some(Duration::from_secs(0)))
                .await
                .unwrap();

            let is_cleared = chain_driver_a
                .chain()
                .query_packet_is_cleared(port_id_a, channel_id_a, &a_to_b_sequences[0])
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_cleared);

            let is_cleared = chain_driver_b
                .chain()
                .query_packet_is_cleared(port_id_b, channel_id_b, &b_to_a_sequences[0])
                .await
                .map_err(Driver::raise_error)?;

            assert!(is_cleared);
        }

        logger
            .log_message(&format!(
                "successfully performed packet clearing between chain `{chain_id_a}` and chain `{chain_id_b}`"
            ))
            .await;

        Ok(())
    }
}

async fn run_one_way_clearing_test<
    Driver,
    Relay,
    SrcChain,
    SrcChainDriver,
    DstChain,
    DstChainDriver,
>(
    src_chain_driver: &SrcChainDriver,
    dst_chain_driver: &DstChainDriver,
    relay: &Relay,
    src_channel_id: &SrcChain::ChannelId,
    src_port_id: &SrcChain::PortId,
    dst_channel_id: &DstChain::ChannelId,
    dst_port_id: &DstChain::PortId,
) -> Result<Vec<SrcChain::Sequence>, Driver::Error>
where
    Driver: CanRaiseAsyncError<Relay::Error>
        + CanRaiseAsyncError<SrcChain::Error>
        + CanRaiseAsyncError<DstChain::Error>,
    Relay: CanRelayReceivePacket<SrcChain = SrcChain, DstChain = DstChain>,
    SrcChainDriver: HasChain<Chain = SrcChain>
        + HasDenom<TransferDenom>
        + HasWallet<UserWallet>
        + CanGenerateRandomAmount,
    DstChainDriver: HasChain<Chain = DstChain> + HasWallet<UserWallet>,
    SrcChain: CanQueryBalance
        + HasDefaultMemo
        + CanIbcTransferToken<DstChain>
        + CanQueryPacketIsCleared<DstChain>
        + CanQueryChainStatus
        + HasPacketSequence<DstChain>
        + HasAmountMethods,
    DstChain: HasWalletType + CanQueryChainStatus + CanQueryPacketIsReceived<SrcChain>,
{
    let sender_wallet = src_chain_driver.wallet(PhantomData::<UserWallet>);

    let sender_address = SrcChain::wallet_address(sender_wallet);

    let receiver_wallet = dst_chain_driver.wallet(PhantomData::<UserWallet>);

    let receiver_address = DstChain::wallet_address(receiver_wallet);

    let denom = src_chain_driver.denom(PhantomData::<TransferDenom>);

    let src_chain = src_chain_driver.chain();
    let dst_chain = dst_chain_driver.chain();

    let initial_balance_sender = src_chain
        .query_balance(sender_address, denom)
        .await
        .map_err(Driver::raise_error)?;

    let transfer_amount_1 = src_chain_driver
        .random_amount(1000, &initial_balance_sender)
        .await;

    let packet_1 = src_chain
        .ibc_transfer_token(
            PhantomData,
            src_channel_id,
            src_port_id,
            sender_wallet,
            receiver_address,
            &transfer_amount_1,
            &src_chain.default_memo(),
            &dst_chain
                .query_chain_status()
                .await
                .map_err(Driver::raise_error)?,
        )
        .await
        .map_err(Driver::raise_error)?;

    let sequence_1 = SrcChain::packet_sequence(&packet_1);

    let src_chain_status = src_chain
        .query_chain_status()
        .await
        .map_err(Driver::raise_error)?;

    let _ = relay
        .relay_receive_packet(SrcChain::chain_status_height(&src_chain_status), &packet_1)
        .await
        .map_err(Driver::raise_error)?;

    let expected_sender_balance =
        SrcChain::subtract_amount(&initial_balance_sender, &transfer_amount_1)
            .map_err(Driver::raise_error)?;

    let current_sender_balance = src_chain
        .query_balance(sender_address, denom)
        .await
        .map_err(Driver::raise_error)?;

    assert_eq!(expected_sender_balance, current_sender_balance);

    // Assert only the ack is pending for packet with sequence 1
    {
        let is_received = dst_chain
            .query_packet_is_received(dst_port_id, dst_channel_id, &sequence_1)
            .await
            .map_err(Driver::raise_error)?;

        assert!(is_received);

        let is_cleared = src_chain
            .query_packet_is_cleared(src_port_id, src_channel_id, &sequence_1)
            .await
            .map_err(Driver::raise_error)?;

        assert!(!is_cleared);
    }

    let transfer_amount_2 = src_chain_driver
        .random_amount(1000, &current_sender_balance)
        .await;

    let packet_2 = src_chain
        .ibc_transfer_token(
            PhantomData,
            src_channel_id,
            src_port_id,
            sender_wallet,
            receiver_address,
            &transfer_amount_2,
            &src_chain.default_memo(),
            &dst_chain
                .query_chain_status()
                .await
                .map_err(Driver::raise_error)?,
        )
        .await
        .map_err(Driver::raise_error)?;

    let sequence_2 = SrcChain::packet_sequence(&packet_2);

    let expected_balance_sender =
        SrcChain::subtract_amount(&current_sender_balance, &transfer_amount_2)
            .map_err(Driver::raise_error)?;

    let current_sender_balance = src_chain
        .query_balance(sender_address, denom)
        .await
        .map_err(Driver::raise_error)?;

    assert_eq!(expected_balance_sender, current_sender_balance);

    // Assert both recv and ack are pending for packet with sequence 2
    {
        let is_received = dst_chain
            .query_packet_is_received(dst_port_id, dst_channel_id, &sequence_2)
            .await
            .map_err(Driver::raise_error)?;

        assert!(!is_received);

        let is_cleared = src_chain
            .query_packet_is_cleared(src_port_id, src_channel_id, &sequence_2)
            .await
            .map_err(Driver::raise_error)?;

        assert!(!is_cleared);
    }

    Ok(vec![sequence_1, sequence_2])
}

use alloc::vec::Vec;
use alloc::{format, vec};
use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::error::ErrorOf;
use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLogMessage;
use hermes_relayer_components::birelay::traits::{CanAutoBiRelay, HasTwoWayRelay};
use hermes_relayer_components::chain::traits::packet::fields::{
    CanReadPacketFields, HasPacketSequence,
};
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::packet_is_cleared::CanQueryPacketIsCleared;
use hermes_relayer_components::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::RelayAt;
use hermes_relayer_components::relay::traits::auto_relayer::CanAutoRelayWithHeights;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasChainTargets, SourceTarget,
};
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain::traits::types::amount::HasAmountMethods;
use hermes_test_components::chain::traits::types::memo::HasDefaultMemo;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::amount::CanGenerateRandomAmount;
use hermes_test_components::chain_driver::traits::fields::denom::{HasDenom, TransferDenom};
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWallet, UserWallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::driver::traits::channel_at::HasChannelIdAt;
use hermes_test_components::driver::traits::types::chain_driver_at::HasChainDriverAt;
use hermes_test_components::driver::traits::types::relay_driver_at::HasRelayDriverAt;
use hermes_test_components::setup::traits::port_id_at::HasPortIdAt;
use hermes_test_components::test_case::traits::test_case::TestCase;

pub struct TestPacketClearing;

impl<Driver, ChainA, ChainB, BiRelay, ChainDriverA, ChainDriverB, RelayDriver, Logger>
    TestCase<Driver> for TestPacketClearing
where
    Driver: HasAsyncErrorType
        + HasLogger<Logger = Logger>
        + HasChainTypeAt<Index<0>, Chain = ChainA>
        + HasChainTypeAt<Index<1>, Chain = ChainB>
        + HasChainDriverAt<Index<0>, ChainDriver = ChainDriverA>
        + HasChainDriverAt<Index<1>, ChainDriver = ChainDriverB>
        + HasRelayDriverAt<Index<0>, Index<1>, RelayDriver = RelayDriver>
        + HasChannelIdAt<Index<0>, Index<1>>
        + HasChannelIdAt<Index<1>, Index<0>>
        + HasPortIdAt<Index<0>, Index<1>>
        + HasPortIdAt<Index<1>, Index<0>>,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasDenom<TransferDenom>
        + HasWallet<UserWallet>
        + CanGenerateRandomAmount,
    ChainDriverB: HasChain<Chain = ChainB>
        + HasWallet<UserWallet>
        + HasDenom<TransferDenom>
        + CanGenerateRandomAmount,
    RelayDriver: HasBiRelayAt<Index<0>, Index<1>, BiRelay = BiRelay>,
    ChainA: HasChainId
        + CanQueryBalance
        + HasAmountMethods
        + CanIbcTransferToken<ChainB>
        + CanQueryPacketIsReceived<ChainB>
        + CanQueryPacketIsCleared<ChainB>
        + CanReadPacketFields<ChainB>
        + HasDefaultMemo
        + CanQueryChainStatus,
    ChainB: HasChainId
        + HasAmountMethods
        + CanQueryBalance
        + CanIbcTransferToken<ChainA>
        + CanQueryPacketIsReceived<ChainA>
        + CanQueryPacketIsCleared<ChainA>
        + CanReadPacketFields<ChainA>
        + HasDefaultMemo
        + CanQueryChainStatus,
    BiRelay: HasTwoWayRelay + CanAutoBiRelay,
    RelayAt<BiRelay, Index<0>, Index<1>>: HasChainTargets<SrcChain = ChainA, DstChain = ChainB>
        + CanRelayReceivePacket
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>,
    RelayAt<BiRelay, Index<1>, Index<0>>: HasChainTargets<SrcChain = ChainB, DstChain = ChainA>
        + CanRelayReceivePacket
        + CanAutoRelayWithHeights<SourceTarget>
        + CanAutoRelayWithHeights<DestinationTarget>,
    Logger: CanLogMessage,
    Driver::Error: From<ChainA::Error>
        + From<ChainB::Error>
        + From<ErrorOf<RelayAt<BiRelay, Index<0>, Index<1>>>>
        + From<ErrorOf<RelayAt<BiRelay, Index<1>, Index<0>>>>,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let logger = driver.logger();

        let chain_driver_a = driver.chain_driver_at(PhantomData::<Index<0>>);

        let chain_driver_b = driver.chain_driver_at(PhantomData::<Index<1>>);

        let relay_driver = driver.relay_driver_at(PhantomData::<(Index<0>, Index<1>)>);

        let birelay = relay_driver.birelay_at();

        let relay_a_to_b = birelay.relay_a_to_b();

        let relay_b_to_a = birelay.relay_b_to_a();

        let chain_a = chain_driver_a.chain();

        let chain_id_a = chain_a.chain_id();

        let chain_b = chain_driver_b.chain();

        let chain_id_b = chain_b.chain_id();

        let channel_id_a = driver.channel_id_at(PhantomData::<(Index<0>, Index<1>)>);

        let port_id_a = driver.port_id_at(PhantomData::<(Index<0>, Index<1>)>);

        let channel_id_b = driver.channel_id_at(PhantomData::<(Index<1>, Index<0>)>);

        let port_id_b = driver.port_id_at(PhantomData::<(Index<1>, Index<0>)>);

        logger
            .log_message(&format!(
                "Test clearing pending recv from chain `{chain_id_a}` to chain `{chain_id_b}` and pending ack from `{chain_id_b}` to `{chain_id_a}`"
            ))
            .await;

        let a_to_b_sequences = run_one_way_clearing_test::<
            Driver,
            RelayAt<BiRelay, Index<0>, Index<1>>,
            ChainA,
            ChainDriverA,
            ChainB,
            ChainDriverB,
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
            RelayAt<BiRelay, Index<1>, Index<0>>,
            ChainB,
            ChainDriverB,
            ChainA,
            ChainDriverA,
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
                .await?;

            assert!(is_received);

            let is_received = chain_driver_b
                .chain()
                .query_packet_is_received(port_id_b, channel_id_b, &a_to_b_sequences[1])
                .await?;

            assert!(is_received);

            birelay
                .auto_bi_relay(Some(Duration::from_secs(15)), Some(Duration::from_secs(0)))
                .await
                .unwrap();

            let is_cleared = chain_driver_a
                .chain()
                .query_packet_is_cleared(port_id_a, channel_id_a, &a_to_b_sequences[0])
                .await?;

            assert!(is_cleared);

            let is_cleared = chain_driver_b
                .chain()
                .query_packet_is_cleared(port_id_b, channel_id_b, &b_to_a_sequences[0])
                .await?;

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
    Driver: HasAsyncErrorType,
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
    Driver::Error: From<SrcChain::Error> + From<DstChain::Error> + From<Relay::Error>,
{
    let sender_wallet = src_chain_driver.wallet(PhantomData::<UserWallet>);

    let sender_address = SrcChain::wallet_address(sender_wallet);

    let receiver_wallet = dst_chain_driver.wallet(PhantomData::<UserWallet>);

    let receiver_address = DstChain::wallet_address(receiver_wallet);

    let denom = src_chain_driver.denom(PhantomData::<TransferDenom>);

    let src_chain = src_chain_driver.chain();
    let dst_chain = dst_chain_driver.chain();

    let initial_balance_sender = src_chain.query_balance(sender_address, denom).await?;

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
            &dst_chain.query_chain_status().await?,
        )
        .await?;

    let sequence_1 = SrcChain::packet_sequence(&packet_1);

    let src_chain_status = src_chain.query_chain_status().await?;

    let _ = relay
        .relay_receive_packet(SrcChain::chain_status_height(&src_chain_status), &packet_1)
        .await?;

    let expected_sender_balance =
        SrcChain::subtract_amount(&initial_balance_sender, &transfer_amount_1)?;

    let current_sender_balance = src_chain.query_balance(sender_address, denom).await?;

    assert_eq!(expected_sender_balance, current_sender_balance);

    // Assert only the ack is pending for packet with sequence 1
    {
        let is_received = dst_chain
            .query_packet_is_received(dst_port_id, dst_channel_id, &sequence_1)
            .await?;

        assert!(is_received);

        let is_cleared = src_chain
            .query_packet_is_cleared(src_port_id, src_channel_id, &sequence_1)
            .await?;

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
            &dst_chain.query_chain_status().await?,
        )
        .await?;

    let sequence_2 = SrcChain::packet_sequence(&packet_2);

    let expected_balance_sender =
        SrcChain::subtract_amount(&current_sender_balance, &transfer_amount_2)?;

    let current_sender_balance = src_chain.query_balance(sender_address, denom).await?;

    assert_eq!(expected_balance_sender, current_sender_balance);

    // Assert both recv and ack are pending for packet with sequence 2
    {
        let is_received = dst_chain
            .query_packet_is_received(dst_port_id, dst_channel_id, &sequence_2)
            .await?;

        assert!(!is_received);

        let is_cleared = src_chain
            .query_packet_is_cleared(src_port_id, src_channel_id, &sequence_2)
            .await?;

        assert!(!is_cleared);
    }

    Ok(vec![sequence_1, sequence_2])
}

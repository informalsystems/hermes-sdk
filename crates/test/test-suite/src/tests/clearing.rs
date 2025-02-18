use alloc::format;
use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLogMessage;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::chain::traits::packet::fields::CanReadPacketFields;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::packet_is_cleared::CanQueryPacketIsCleared;
use hermes_relayer_components::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayAt;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::RelayAt;
use hermes_relayer_components::relay::traits::auto_relayer::CanAutoRelayWithHeights;
use hermes_relayer_components::relay::traits::chains::HasRelayChainTypes;
use hermes_relayer_components::relay::traits::target::{HasSourceTargetChainTypes, SourceTarget};
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::amount::CanConvertIbcTransferredAmount;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain::traits::types::amount::HasAmountMethods;
use hermes_test_components::chain::traits::types::memo::HasDefaultMemo;
use hermes_test_components::chain_driver::traits::fields::amount::CanGenerateRandomAmount;
use hermes_test_components::chain_driver::traits::fields::denom_at::{HasDenomAt, TransferDenom};
use hermes_test_components::chain_driver::traits::fields::wallet::{HasWalletAt, UserWallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::driver::traits::channel_at::HasChannelAt;
use hermes_test_components::driver::traits::types::chain_driver_at::HasChainDriverAt;
use hermes_test_components::driver::traits::types::relay_driver_at::HasRelayDriverAt;
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
        + HasChannelAt<Index<0>, Index<1>>
        + HasChannelAt<Index<1>, Index<0>>,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasDenomAt<TransferDenom, Index<0>>
        + HasWalletAt<UserWallet, Index<0>>
        + HasWalletAt<UserWallet, Index<1>>
        + CanGenerateRandomAmount,
    ChainDriverB:
        HasChain<Chain = ChainB> + HasWalletAt<UserWallet, Index<0>> + CanGenerateRandomAmount,
    RelayDriver: HasBiRelayAt<Index<0>, Index<1>, BiRelay = BiRelay>,
    ChainA: HasIbcChainTypes<ChainB>
        + HasChainId
        + HasOutgoingPacketType<ChainB>
        + CanQueryChainHeight
        + CanQueryBalance
        + HasAmountMethods
        + CanConvertIbcTransferredAmount<ChainB>
        + CanIbcTransferToken<ChainB>
        + CanQueryPacketIsReceived<ChainB>
        + CanQueryPacketIsCleared<ChainB>
        + CanReadPacketFields<ChainB>
        + HasDefaultMemo,
    ChainB: HasIbcChainTypes<ChainA>
        + HasChainId
        + HasOutgoingPacketType<ChainA>
        + CanQueryChainHeight
        + HasAmountMethods
        + CanQueryBalance
        + CanIbcTransferToken<ChainA>
        + CanConvertIbcTransferredAmount<ChainA>
        + CanQueryPacketIsReceived<ChainA>
        + CanQueryPacketIsCleared<ChainA>
        + CanReadPacketFields<ChainA>
        + HasDefaultMemo,
    BiRelay: HasTwoWayRelay,
    RelayAt<BiRelay, Index<0>, Index<1>>: HasRelayChainTypes<SrcChain = ChainA, DstChain = ChainB>
        + HasSourceTargetChainTypes
        + CanAutoRelayWithHeights<SourceTarget>,
    RelayAt<BiRelay, Index<1>, Index<0>>: HasRelayChainTypes<SrcChain = ChainB, DstChain = ChainA>
        + HasSourceTargetChainTypes
        + CanAutoRelayWithHeights<SourceTarget>,
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

        let birelay = relay_driver.birelay_at(PhantomData);

        let relay_a_to_b = birelay.relay_a_to_b();

        let relay_b_to_a = birelay.relay_b_to_a();

        let chain_a = chain_driver_a.chain();

        let chain_id_a = chain_a.chain_id();

        let chain_b = chain_driver_b.chain();

        let chain_id_b = chain_b.chain_id();

        let wallet_a1 = chain_driver_a.wallet_at(UserWallet, PhantomData::<Index<0>>);

        let address_a1 = ChainA::wallet_address(wallet_a1);

        let wallet_b = chain_driver_b.wallet_at(UserWallet, PhantomData::<Index<0>>);

        let address_b = ChainB::wallet_address(wallet_b);

        let denom_a = chain_driver_a.denom_at(TransferDenom, PhantomData::<Index<0>>);

        let balance_a1 = chain_a.query_balance(address_a1, denom_a).await?;

        let a_to_b_amount = chain_driver_a.random_amount(1000, &balance_a1).await;

        let channel_id_a = driver.channel_id_at(PhantomData::<(Index<0>, Index<1>)>);

        let port_id_a = driver.port_id_at(PhantomData::<(Index<0>, Index<1>)>);

        let channel_id_b = driver.channel_id_at(PhantomData::<(Index<1>, Index<0>)>);

        let port_id_b = driver.port_id_at(PhantomData::<(Index<1>, Index<0>)>);

        let height_a1 = chain_a.query_chain_height().await?;

        logger
            .log_message(&format!(
                "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
                chain_id_a, chain_id_b, a_to_b_amount, denom_a
            ))
            .await;

        let packet_a = chain_a
            .ibc_transfer_token(
                channel_id_a,
                port_id_a,
                wallet_a1,
                address_b,
                &a_to_b_amount,
                &chain_a.default_memo(),
            )
            .await?;

        let sequence_a = ChainA::packet_sequence(&packet_a);

        let balance_a2 = ChainA::subtract_amount(&balance_a1, &a_to_b_amount)?;

        let balance_a3 = chain_a.query_balance(address_a1, denom_a).await?;

        assert_eq!(balance_a2, balance_a3);

        let height_a2 = chain_a.query_chain_height().await?;

        {
            let is_received = chain_b
                .query_packet_is_received(port_id_b, channel_id_b, &sequence_a)
                .await?;

            assert!(!is_received);

            let is_cleared = chain_a
                .query_packet_is_cleared(port_id_a, channel_id_a, &sequence_a)
                .await?;

            assert!(!is_cleared);
        }

        relay_a_to_b
            .auto_relay_with_heights(SourceTarget, &height_a1, Some(&height_a2))
            .await?;

        let balance_b1 = ChainB::ibc_transfer_amount_from(&a_to_b_amount, channel_id_b, port_id_b)?;

        {
            let balance_b2 = chain_b
                .query_balance(address_b, ChainB::amount_denom(&balance_b1))
                .await?;

            assert_eq!(balance_b2, balance_b1);
        }

        {
            let is_received = chain_b
                .query_packet_is_received(port_id_b, channel_id_b, &sequence_a)
                .await?;

            assert!(is_received);

            let is_cleared = chain_a
                .query_packet_is_cleared(port_id_a, channel_id_a, &sequence_a)
                .await?;

            assert!(is_cleared);
        }

        let height_b1 = chain_b.query_chain_height().await?;

        let wallet_a2 = chain_driver_a.wallet_at(UserWallet, PhantomData::<Index<1>>);

        let address_a2 = ChainA::wallet_address(wallet_a2);

        let b_to_a_amount = chain_driver_b.random_amount(500, &balance_b1).await;

        logger
            .log_message(&format!(
                "Sending IBC transfer from chain {} to chain {} with amount of {}",
                chain_id_b, chain_id_a, b_to_a_amount,
            ))
            .await;

        let packet_b = chain_b
            .ibc_transfer_token(
                channel_id_b,
                port_id_b,
                wallet_b,
                address_a2,
                &b_to_a_amount,
                &chain_b.default_memo(),
            )
            .await?;

        let sequence_b = ChainB::packet_sequence(&packet_b);

        let balance_b2 = ChainB::subtract_amount(&balance_b1, &b_to_a_amount)?;

        let denom_b = ChainB::amount_denom(&balance_b1);

        let balance_b3 = chain_b.query_balance(address_b, denom_b).await?;

        assert_eq!(balance_b2, balance_b3);

        let balance_a4 = chain_a.query_balance(address_a2, denom_a).await?;

        let balance_a5 = ChainA::add_amount(
            &balance_a4,
            &ChainA::transmute_counterparty_amount(&b_to_a_amount, denom_a),
        )?;

        let height_b2 = chain_b.query_chain_height().await?;

        {
            let is_received = chain_a
                .query_packet_is_received(port_id_a, channel_id_a, &sequence_b)
                .await?;

            assert!(!is_received);

            let is_cleared = chain_b
                .query_packet_is_cleared(port_id_b, channel_id_b, &sequence_b)
                .await?;

            assert!(!is_cleared);
        }

        relay_b_to_a
            .auto_relay_with_heights(SourceTarget, &height_b1, Some(&height_b2))
            .await?;

        {
            let balance_a6 = chain_a
                .query_balance(address_a2, ChainA::amount_denom(&balance_a5))
                .await?;

            assert_eq!(balance_a6, balance_a5);
        }

        {
            let is_received = chain_a
                .query_packet_is_received(port_id_a, channel_id_a, &sequence_b)
                .await?;

            assert!(is_received);

            let is_cleared = chain_b
                .query_packet_is_cleared(port_id_b, channel_id_b, &sequence_b)
                .await?;

            assert!(is_cleared);
        }

        logger
            .log_message(&format!(
                "successfully performed reverse IBC transfer from chain {} back to chain {}",
                chain_id_b, chain_id_a,
            ))
            .await;

        Ok(())
    }
}

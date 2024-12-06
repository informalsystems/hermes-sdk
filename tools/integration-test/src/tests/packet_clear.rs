use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelay;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::CanQueryPacketAcknowledgements;
use hermes_relayer_components::chain::traits::queries::packet_commitments::CanQueryPacketCommitments;
use hermes_relayer_components::chain::traits::queries::send_packets::CanQuerySendPackets;
use hermes_relayer_components::chain::traits::queries::unreceived_acks_sequences::CanQueryUnreceivedAcksSequences;
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::CanQueryUnreceivedPacketSequences;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use hermes_relayer_components::relay::traits::chains::{HasDstChain, HasSrcChain};
use hermes_relayer_components::relay::traits::packet_clearer::CanClearPackets;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use ibc_relayer::config::PacketFilter;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::Height;
use ibc_test_framework::framework::next::chain::{HasTwoChains, HasTwoChannels};
use ibc_test_framework::prelude::*;
use ibc_test_framework::util::random::random_u64_range;

use crate::tests::context::build_cosmos_relay_context;

#[test]
fn test_ibc_clear_packet_next() -> Result<(), Error> {
    run_binary_channel_test(&IbcClearPacketTest)
}

#[test]
fn test_ibc_clear_ack_next() -> Result<(), Error> {
    run_binary_channel_test(&IbcClearAckTest)
}

pub struct IbcClearPacketTest;

impl TestOverrides for IbcClearPacketTest {
    fn should_spawn_supervisor(&self) -> bool {
        false
    }
}

impl BinaryChannelTest for IbcClearPacketTest {
    fn run<Context>(&self, relayer: RelayerDriver, context: &Context) -> Result<(), Error>
    where
        Context: HasTwoChains + HasTwoChannels,
    {
        let chains = context.chains();
        let cloned_channel = context.channel().clone();
        let channel = context.channel().clone();
        let pf: PacketFilter = PacketFilter::default();

        let relay_context = build_cosmos_relay_context(&relayer.config, chains, pf)?;

        let relay_a_to_b = relay_context.relay_a_to_b();
        let relay_b_to_a = relay_context.relay_b_to_a();
        let chain_a = relay_a_to_b.src_chain();
        let chain_b = relay_a_to_b.dst_chain();

        let runtime = chains.node_a.value().chain_driver.runtime.as_ref();

        let denom_a = chains.node_a.denom();

        let wallet_a = chains.node_a.wallets().user1().cloned();
        let wallet_b = chains.node_b.wallets().user1().cloned();

        let balance_a = chains
            .node_a
            .chain_driver()
            .query_balance(&wallet_a.address(), &denom_a)?;

        let a_to_b_amount = random_u64_range(1000, 5000);

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chains.chain_id_a(),
            chains.chain_id_b(),
            a_to_b_amount,
            denom_a
        );

        chains.node_a.chain_driver().ibc_transfer_token(
            &channel.port_a.as_ref(),
            &channel.channel_id_a.as_ref(),
            &wallet_a.as_ref(),
            &wallet_b.address(),
            &denom_a.with_amount(a_to_b_amount).as_ref(),
        )?;

        let denom_b = derive_ibc_denom(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &denom_a,
        )?;

        runtime.block_on(async {
            info!("Assert query packet commitments works as expected");

            let (src_commitments, src_height): (Vec<Sequence>, Height) =
                <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                    chain_a,
                    channel.channel_id_a.value(),
                    channel.port_a.value(),
                )
                .await
                .unwrap();

            assert_eq!(src_commitments, vec!(Sequence::from(1)));

            let (dst_commitments, dst_height): (Vec<Sequence>, Height) =
                <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                    chain_b,
                    channel.channel_id_b.value(),
                    channel.port_b.value(),
                )
                .await
                .unwrap();

            assert_eq!(dst_commitments, vec!());

            info!("Assert query unreceived packet sequences works as expected");

            let unreceived_packet_sequences: Vec<Sequence> =
                <CosmosChain as CanQueryUnreceivedPacketSequences<
                    CosmosChain,
                >>::query_unreceived_packet_sequences(
                    chain_a,
                    channel.channel_id_a.value(),
                    channel.port_a.value(),
                    &src_commitments,
                )
                .await
                .unwrap();

            assert_eq!(unreceived_packet_sequences, vec!(Sequence::from(1)));

            let unreceived_packet_sequences: Vec<Sequence> =
                <CosmosChain as CanQueryUnreceivedPacketSequences<
                    CosmosChain,
                >>::query_unreceived_packet_sequences(
                    chain_b,
                    channel.channel_id_b.value(),
                    channel.port_b.value(),
                    &src_commitments,
                )
                .await
                .unwrap();

            assert_eq!(unreceived_packet_sequences, vec!(Sequence::from(1)));

            info!("Assert query unreceived packets works as expected");

            let send_packets = <CosmosChain as CanQuerySendPackets<
                CosmosChain,
            >>::query_send_packets_from_sequences(
                chain_a,
                channel.channel_id_a.value(),
                channel.port_a.value(),
                channel.channel_id_b.value(),
                channel.port_b.value(),
                &unreceived_packet_sequences,
                &src_height,
            )
            .await
            .unwrap();

            assert_eq!(send_packets.len(), 1);

            let send_packets = <CosmosChain as CanQuerySendPackets<
                CosmosChain,
            >>::query_send_packets_from_sequences(
                chain_b,
                channel.channel_id_b.value(),
                channel.port_b.value(),
                channel.channel_id_a.value(),
                channel.port_a.value(),
                &unreceived_packet_sequences,
                &dst_height,
            )
            .await;

            assert!(
                send_packets.is_err(),
                "There should be no send packets from Chain B"
            );

            let _ = relay_b_to_a
                .clear_packets(
                    channel.channel_id_b.value(),
                    channel.port_b.value(),
                    channel.channel_id_a.value(),
                    channel.port_a.value(),
                )
                .await;

            info!("Clear packets from B to A should not clear the pending packet from A to B");

            let amount = chains
                .node_a
                .chain_driver()
                .query_balance(&wallet_a.address(), &balance_a.denom())
                .unwrap();

            assert_eq!(
                amount.value().amount,
                (balance_a.clone() - a_to_b_amount).amount()
            );

            let amount = chains
                .node_b
                .chain_driver()
                .query_balance(&wallet_b.address(), &denom_b.as_ref())
                .unwrap();

            assert_eq!(amount.value().amount, denom_b.with_amount(0u64).amount());

            let _ = relay_a_to_b
                .clear_packets(
                    cloned_channel.channel_id_a.value(),
                    cloned_channel.port_a.value(),
                    cloned_channel.channel_id_b.value(),
                    cloned_channel.port_b.value(),
                )
                .await;

            info!("Clear packet from A to B should clear the pending packet");

            let amount = chains
                .node_a
                .chain_driver()
                .query_balance(&wallet_a.address(), &balance_a.denom())
                .unwrap();

            assert_eq!(
                amount.value().amount,
                (balance_a.clone() - a_to_b_amount).amount()
            );

            let amount = chains
                .node_b
                .chain_driver()
                .query_balance(&wallet_b.address(), &denom_b.as_ref())
                .unwrap();

            assert_eq!(
                amount.value().amount,
                denom_b.with_amount(a_to_b_amount).amount()
            );
        });

        Ok(())
    }
}

pub struct IbcClearAckTest;

impl TestOverrides for IbcClearAckTest {
    fn should_spawn_supervisor(&self) -> bool {
        false
    }
}

impl BinaryChannelTest for IbcClearAckTest {
    fn run<Context>(&self, relayer: RelayerDriver, context: &Context) -> Result<(), Error>
    where
        Context: HasTwoChains + HasTwoChannels,
    {
        let chains = context.chains();
        let cloned_channel = context.channel().clone();
        let channel = context.channel().clone();
        let pf: PacketFilter = PacketFilter::default();

        let relay_context = build_cosmos_relay_context(&relayer.config, chains, pf)?;

        let relay_a_to_b = relay_context.relay_a_to_b();
        let chain_a = relay_a_to_b.src_chain();
        let chain_b = relay_a_to_b.dst_chain();

        let runtime = chains.node_a.value().chain_driver.runtime.as_ref();

        let denom_a = chains.node_a.denom();

        let wallet_a = chains.node_a.wallets().user1().cloned();
        let wallet_b = chains.node_b.wallets().user1().cloned();

        let balance_a = chains
            .node_a
            .chain_driver()
            .query_balance(&wallet_a.address(), &denom_a)?;

        let a_to_b_amount = random_u64_range(1000, 5000);

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chains.chain_id_a(),
            chains.chain_id_b(),
            a_to_b_amount,
            denom_a
        );

        chains.node_a.chain_driver().ibc_transfer_token(
            &channel.port_a.as_ref(),
            &channel.channel_id_a.as_ref(),
            &wallet_a.as_ref(),
            &wallet_b.address(),
            &denom_a.with_amount(a_to_b_amount).as_ref(),
        )?;

        let denom_b = derive_ibc_denom(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &denom_a,
        )?;

        runtime.block_on(async {
            // Will only clear the receive packet
            let (src_commitments, src_height): (Vec<Sequence>, Height) =
                <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                    chain_a,
                    channel.channel_id_a.value(),
                    channel.port_a.value(),
                )
                .await
                .unwrap();

            let unreceived_packet_sequences: Vec<Sequence> =
                <CosmosChain as CanQueryUnreceivedPacketSequences<
                    CosmosChain,
                >>::query_unreceived_packet_sequences(
                    chain_b,
                    channel.channel_id_b.value(),
                    channel.port_b.value(),
                    &src_commitments,
                )
                .await
                .unwrap();

            let send_packets =<CosmosChain as CanQuerySendPackets<CosmosChain>>::query_send_packets_from_sequences(
                    chain_a,
                    channel.channel_id_a.value(),
                    channel.port_a.value(),
                    channel.channel_id_b.value(),
                    channel.port_b.value(),
                    &unreceived_packet_sequences,
                    &src_height,
                )
                .await
                .unwrap();

            let src_chain_status = chain_a
                .query_chain_status()
                .await
                .unwrap();

            for packet in send_packets.iter() {
                let _write_ack = relay_a_to_b
                    .relay_receive_packet(
                        <CosmosChain as HasChainStatusType>::chain_status_height(&src_chain_status),
                        packet,
                    )
                    .await.unwrap();
            }

            info!("The receive packet relaying should have escrowed the tokens");

            let amount = chains
                .node_a
                .chain_driver()
                .query_balance(&wallet_a.address(), &balance_a.denom())
                .unwrap();

            assert_eq!(
                amount.value().amount,
                (balance_a.clone() - a_to_b_amount).amount()
            );

            let amount = chains
                .node_b
                .chain_driver()
                .query_balance(&wallet_b.address(), &denom_b.as_ref())
                .unwrap();

            assert_eq!(
                amount.value().amount,
                denom_b.with_amount(a_to_b_amount).amount()
            );

            info!("Assert query packet commitments works as expected");

            let (src_commitments, _): (Vec<Sequence>, Height) =
                <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                    chain_a,
                    channel.channel_id_a.value(),
                    channel.port_a.value(),
                )
                .await
                .unwrap();

            assert_eq!(src_commitments, vec!(Sequence::from(1)));

            info!("Assert packet clearing clear the pending acks");

            let acks_and_height_on_counterparty = <CosmosChain as CanQueryPacketAcknowledgements<CosmosChain>>::query_packet_acknowlegements(
                chain_b,
                channel.channel_id_b.value(),
                channel.port_b.value(),
                &src_commitments
            ).await.unwrap();

            assert!(acks_and_height_on_counterparty.is_some());

            info!("Assert query unreceived acknowledgment sequences works as expected");

            let unreceived_ack_sequences =
                <CosmosChain as CanQueryUnreceivedAcksSequences<
                    CosmosChain,
                >>::query_unreceived_acknowledgments_sequences(
                    chain_a,
                    channel.channel_id_a.value(),
                    channel.port_a.value(),
                    &acks_and_height_on_counterparty.clone().unwrap().0,
                )
                .await
                .unwrap();

            assert_eq!(unreceived_ack_sequences, vec!(Sequence::from(1)));

            let _ = relay_a_to_b
                .clear_packets(
                    cloned_channel.channel_id_a.value(),
                    cloned_channel.port_a.value(),
                    cloned_channel.channel_id_b.value(),
                    cloned_channel.port_b.value(),
                )
                .await;

            let acks_and_height_on_counterparty = <CosmosChain as CanQueryPacketAcknowledgements<CosmosChain>>::query_packet_acknowlegements(
                chain_b,
                channel.channel_id_b.value(),
                channel.port_b.value(),
                &src_commitments
            ).await.unwrap();

            let unreceived_ack_sequences =
                <CosmosChain as CanQueryUnreceivedAcksSequences<
                    CosmosChain,
                >>::query_unreceived_acknowledgments_sequences(
                    chain_a,
                    channel.channel_id_a.value(),
                    channel.port_a.value(),
                    &acks_and_height_on_counterparty.clone().unwrap().0,
                )
                .await
                .unwrap();

            assert_eq!(unreceived_ack_sequences, vec!());
        });

        Ok(())
    }
}

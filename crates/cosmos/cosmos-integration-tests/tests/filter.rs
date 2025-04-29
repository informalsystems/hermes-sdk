#![recursion_limit = "256"]

use core::marker::PhantomData;
use std::collections::HashMap;

use hermes_core::relayer_components::chain::traits::{
    CanQueryChainStatus, CanQueryPacketIsReceived,
};
use hermes_core::test_components::chain::traits::{
    CanAssertEventualAmount, CanConvertIbcTransferredAmount, CanIbcTransferToken, CanQueryBalance,
    HasDefaultMemo,
};
use hermes_core::test_components::chain_driver::traits::CanGenerateRandomAmount;
use hermes_core::test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_cosmos_core::chain_components::types::PacketFilterConfig;
use hermes_cosmos_core::test_components::chain::types::Amount;
use hermes_cosmos_integration_tests::contexts::CosmosBinaryChannelTestDriver;
use hermes_cosmos_integration_tests::init::{init_preset_bootstraps, init_test_runtime};
use hermes_cosmos_relayer::contexts::CosmosChain;
use hermes_error::types::Error;
use ibc::core::host::types::identifiers::{ChannelId, PortId};

#[test]
fn packet_filter_test() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let mut filter_map = HashMap::new();
        filter_map.insert((ChannelId::new(0), PortId::transfer()), false);
        let packet_filter = PacketFilterConfig::new(filter_map);
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, packet_filter).await?;

        let balance_a = setup
            .chain_driver_a
            .chain
            .query_balance(
                &setup.chain_driver_a.user_wallet_a.address,
                &setup.chain_driver_a.genesis_config.transfer_denom,
            )
            .await?;

        let a_to_b_amount = setup.chain_driver_a.random_amount(1000, &balance_a).await;

        let balance_after_escrow = Amount::new(
            balance_a.quantity - a_to_b_amount.quantity,
            balance_a.denom.clone(),
        );

        let _relayer = setup.relay_driver.run_relayer_in_background().await?;

        let packet = setup
            .chain_driver_a
            .chain
            .ibc_transfer_token(
                PhantomData::<CosmosChain>,
                &setup.channel_id_a,
                &setup.port_id_a,
                &setup.chain_driver_a.user_wallet_a,
                &setup.chain_driver_b.user_wallet_b.address,
                &a_to_b_amount,
                &setup.chain_driver_a.chain.default_memo(),
                &setup.chain_driver_b.chain.query_chain_status().await?,
            )
            .await?;

        // Assert tokens have been escrowed
        setup
            .chain_driver_a
            .chain
            .assert_eventual_amount(
                &setup.chain_driver_a.user_wallet_a.address,
                &balance_after_escrow,
            )
            .await?;

        // Wait for a bit
        tokio::time::sleep(core::time::Duration::from_secs(5)).await;

        let is_received =
            <CosmosChain as CanQueryPacketIsReceived<CosmosChain>>::query_packet_is_received(
                &setup.chain_driver_b.chain,
                &setup.port_id_b,
                &setup.channel_id_b,
                &packet.seq_on_a,
            )
            .await?;

        assert!(!is_received);

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

#[test]
fn no_packet_filter_test() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

        let _relayer = setup.relay_driver.run_relayer_in_background().await?;

        let balance_a = setup
            .chain_driver_a
            .chain
            .query_balance(
                &setup.chain_driver_a.user_wallet_a.address,
                &setup.chain_driver_a.genesis_config.transfer_denom,
            )
            .await?;

        let a_to_b_amount = setup.chain_driver_a.random_amount(1000, &balance_a).await;

        let balance_b = setup
            .chain_driver_b
            .chain
            .ibc_transfer_amount_from(
                PhantomData::<CosmosChain>,
                &a_to_b_amount,
                &setup.channel_id_b,
                &setup.port_id_b,
            )
            .await?;

        let balance_after_escrow = Amount::new(
            balance_a.quantity - a_to_b_amount.quantity,
            balance_a.denom.clone(),
        );

        let packet = setup
            .chain_driver_a
            .chain
            .ibc_transfer_token(
                PhantomData::<CosmosChain>,
                &setup.channel_id_a,
                &setup.port_id_a,
                &setup.chain_driver_a.user_wallet_a,
                &setup.chain_driver_b.user_wallet_b.address,
                &a_to_b_amount,
                &setup.chain_driver_a.chain.default_memo(),
                &setup.chain_driver_b.chain.query_chain_status().await?,
            )
            .await?;

        // Assert tokens have been escrowed
        setup
            .chain_driver_a
            .chain
            .assert_eventual_amount(
                &setup.chain_driver_a.user_wallet_a.address,
                &balance_after_escrow,
            )
            .await?;

        // Assert there are no pending packets and tokens have been transferred
        setup
            .chain_driver_b
            .chain
            .assert_eventual_amount(&setup.chain_driver_b.user_wallet_b.address, &balance_b)
            .await?;

        let is_received =
            <CosmosChain as CanQueryPacketIsReceived<CosmosChain>>::query_packet_is_received(
                &setup.chain_driver_b.chain,
                &setup.port_id_b,
                &setup.channel_id_b,
                &packet.seq_on_a,
            )
            .await?;

        assert!(is_received);

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

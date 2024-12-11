#![recursion_limit = "256"]

use hermes_cosmos_integration_tests::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use hermes_cosmos_integration_tests::init::{
    build_tracing_subscriber, init_preset_bootstraps, init_test_runtime,
};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_error::types::Error;
use hermes_relayer_components::relay::traits::packet_clearer::CanClearPackets;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::chain::traits::types::memo::HasDefaultMemo;
use hermes_test_components::chain_driver::traits::fields::amount::CanGenerateRandomAmount;

#[test]
fn timeout_transfer_test() -> Result<(), Error> {
    let subscriber = build_tracing_subscriber();
    let _ = tracing::subscriber::set_default(subscriber);

    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

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

        <CosmosChain as CanIbcTransferToken<CosmosChain>>::ibc_transfer_token(
            &setup.chain_driver_a.chain,
            &setup.channel_id_a,
            &setup.port_id_a,
            &setup.chain_driver_a.user_wallet_a,
            &setup.chain_driver_b.user_wallet_b.address,
            &a_to_b_amount,
            &setup.chain_driver_a.chain.default_memo(),
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

        // Wait for timeout before asserting packets are cleared
        // Timeout for tests is set in crates/cosmos/cosmos-test-components/src/chain/components.rs
        tokio::time::sleep(core::time::Duration::from_secs(95)).await;

        setup
            .relay_driver
            .birelay
            .relay_a_to_b
            .clear_packets(
                &setup.channel_id_a,
                &setup.port_id_a,
                &setup.channel_id_b,
                &setup.port_id_b,
            )
            .await?;

        // Assert tokens have been unescrowed after relaying timeout
        setup
            .chain_driver_a
            .chain
            .assert_eventual_amount(&setup.chain_driver_a.user_wallet_a.address, &balance_a)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

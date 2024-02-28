#![recursion_limit = "256"]

use core::time::Duration;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use borsh::BorshSerialize;
use eyre::{eyre, Error};
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_components::transaction::traits::components::tx_response_querier::CanQueryTxResponse;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_client_components::sovereign::traits::rollup::publish_batch::CanPublishTransactionBatch;
use hermes_sovereign_client_components::sovereign::traits::rollup::queries::events::CanQueryEventsByEventIds;
use hermes_sovereign_client_components::sovereign::types::message::SovereignMessage;
use hermes_sovereign_client_components::sovereign::types::messages::bank::{
    BankMessage, CoinFields,
};
use hermes_sovereign_client_components::sovereign::types::rpc::tx_hash::TxHash;
use hermes_sovereign_client_components::sovereign::utils::encode_tx::encode_and_sign_sovereign_tx;
use hermes_sovereign_integration_tests::contexts::bootstrap::SovereignBootstrap;
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_sovereign_test_components::types::amount::SovereignAmount;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use tokio::runtime::Builder;
use tokio::time::sleep;

#[test]
fn test_sovereign_bootstrap() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let store_postfix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    let store_dir = std::env::current_dir()?.join(format!("test-data/{store_postfix}"));

    let celestia_bootstrap = CelestiaBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        chain_store_dir: store_dir.join("chains"),
        bridge_store_dir: store_dir.join("bridges"),
    };

    let sovereign_bootstrap = SovereignBootstrap {
        runtime: runtime.clone(),
        rollup_store_dir: format!("./test-data/{store_postfix}/rollups").into(),
        rollup_command_path: "node".into(),
        account_prefix: "sov".into(),
    };

    tokio_runtime.block_on(async move {
        let chain_driver = celestia_bootstrap.bootstrap_chain("private").await?;

        // println!("chain home dir: {:?}", chain_driver.chain_node_config.chain_home_dir);
        // tokio::time::sleep(core::time::Duration::from_secs(30)).await;

        let bridge_driver = celestia_bootstrap.bootstrap_bridge(&chain_driver).await?;

        let rollup_driver = sovereign_bootstrap
            .bootstrap_rollup(&chain_driver, &bridge_driver, "test-rollup")
            .await?;

        {
            // Temporary test to check that rollup driver is bootstrapped properly

            let rollup = &rollup_driver.rollup;

            let wallet_a = rollup_driver
                .wallets
                .get("user-a")
                .ok_or_else(|| eyre!("expect user-a wallet"))?;

            let wallet_b = rollup_driver
                .wallets
                .get("user-b")
                .ok_or_else(|| eyre!("expect user-a wallet"))?;

            let transfer_denom = &rollup_driver.genesis_config.transfer_token_address;

            let address_a = &wallet_a.address.address;
            let address_b = &wallet_b.address.address;
            let transfer_denom = &transfer_denom.address;

            {
                let amount = rollup.query_balance(address_a, transfer_denom).await?;

                assert_eq!(amount.quantity, 1_000_000_000_000);
            }

            {
                let amount = rollup.query_balance(address_b, transfer_denom).await?;

                assert_eq!(amount.quantity, 1_000_000_000_000);
            }

            let message = SovereignMessage::Bank(BankMessage::Transfer {
                to: wallet_b.address.address_bytes.clone(),
                coins: CoinFields {
                    amount: 1000,
                    token_address: rollup_driver
                        .genesis_config
                        .transfer_token_address
                        .address_bytes
                        .clone(),
                },
            });

            let message_bytes = message.try_to_vec()?;

            let tx_bytes = encode_and_sign_sovereign_tx(
                &wallet_a.signing_key,
                message_bytes.clone(),
                0,
                0,
                0,
                0,
            )?;

            let tx_hash = TxHash::from_signed_tx_bytes(&tx_bytes);

            {
                let response = rollup.query_tx_response(&tx_hash).await?;

                assert!(response.is_none());
            }

            rollup.publish_transaction_batch(&[tx_bytes]).await?;

            rollup
                .assert_eventual_amount(
                    address_a,
                    &SovereignAmount {
                        quantity: 999_999_999_000,
                        denom: transfer_denom.clone(),
                    },
                )
                .await?;

            rollup
                .assert_eventual_amount(
                    address_b,
                    &SovereignAmount {
                        quantity: 1_000_000_001_000,
                        denom: transfer_denom.clone(),
                    },
                )
                .await?;

            {
                let response = rollup.query_tx_response(&tx_hash).await?;

                println!("querty tx hash {} response: {:?}", tx_hash, response);
            }

            let message = SovereignMessage::Bank(BankMessage::CreateToken {
                salt: 0,
                token_name: "test".into(),
                initial_balance: 1000,
                minter_address: wallet_a.address.address_bytes.clone(),
                authorized_minters: Vec::new(),
            });

            let message_bytes = message.try_to_vec()?;

            let tx_bytes = encode_and_sign_sovereign_tx(
                &wallet_a.signing_key,
                message_bytes.clone(),
                0,
                0,
                0,
                1,
            )?;

            let tx_hash = TxHash::from_signed_tx_bytes(&tx_bytes);

            {
                let response = rollup.query_tx_response(&tx_hash).await?;

                assert!(response.is_none());
            }

            rollup.publish_transaction_batch(&[tx_bytes]).await?;

            sleep(Duration::from_secs(2)).await;

            let response = rollup.query_tx_response(&tx_hash).await?.unwrap();

            println!("querty tx hash {} response: {:?}", tx_hash, response);

            let event_numbers: Vec<u64> = response.event_range.collect();

            println!("event numbers: {:?}", event_numbers);

            {
                let events = rollup.query_events_by_event_ids(&event_numbers).await?;

                println!("querty events response: {:?}", events);
            }
        }
        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

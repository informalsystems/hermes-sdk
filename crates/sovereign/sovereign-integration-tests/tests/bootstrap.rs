#![recursion_limit = "256"]

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use eyre::eyre;
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::transaction::traits::send_messages_with_signer::CanSendMessagesWithSigner;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_integration_tests::contexts::sovereign_bootstrap::SovereignBootstrap;
use hermes_sovereign_rollup_components::types::message::SovereignMessage;
use hermes_sovereign_rollup_components::types::messages::bank::{BankMessage, CoinFields};
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use tokio::runtime::Builder;

#[test]
fn test_sovereign_bootstrap() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let store_postfix = format!(
        "{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        rand::random::<u64>()
    );

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
        rollup_command_path: "rollup".into(),
        account_prefix: "sov".into(),
    };

    tokio_runtime.block_on(async move {
        let chain_driver = celestia_bootstrap.bootstrap_chain("private").await?;

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

            assert_eq!(
                rollup
                    .query_balance(address_a, transfer_denom)
                    .await?
                    .quantity,
                1_000_000_000_000
            );
            assert_eq!(
                rollup
                    .query_balance(address_b, transfer_denom)
                    .await?
                    .quantity,
                1_000_000_000_000
            );

            {
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

                let events = rollup
                    .send_messages_with_signer(&wallet_a.signing_key, &[message])
                    .await?;

                println!("TokenTransfer events: {:?}", events);

                tokio::time::sleep(core::time::Duration::from_secs(1)).await;

                assert_eq!(
                    rollup
                        .query_balance(address_a, transfer_denom)
                        .await?
                        .quantity,
                    999_999_999_000
                );

                assert_eq!(
                    rollup
                        .query_balance(address_b, transfer_denom)
                        .await?
                        .quantity,
                    1_000_000_001_000
                );
            }

            {
                let message = SovereignMessage::Bank(BankMessage::CreateToken {
                    salt: 0,
                    token_name: "test".into(),
                    initial_balance: 1000,
                    minter_address: wallet_a.address.address_bytes.clone(),
                    authorized_minters: Vec::new(),
                });

                let events = rollup
                    .send_messages_with_signer(&wallet_a.signing_key, &[message])
                    .await?;

                println!("CreateToken events: {:?}", events);
            }
        }
        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

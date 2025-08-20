#![recursion_limit = "256"]

use hermes_core::test_components::test_case::traits::test_case::TestCase;
use hermes_cosmos_integration_tests::contexts::CosmosBinaryChannelTestDriver;
use hermes_cosmos_integration_tests::init::{init_preset_bootstraps, init_test_runtime};
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::batch_transactions::TestBatchIbcTransfer;
use hermes_ibc_test_suite::tests::clearing::TestPacketClearing;
use hermes_ibc_test_suite::tests::misebehaviour::TestMisbehaviourDetection;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;

#[test]
fn test_ibc_transfer() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

        TestIbcTransfer::default().run_test(&setup).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

#[test]
fn test_batch_ibc_transfer() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

        TestBatchIbcTransfer::default().run_test(&setup).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

#[test]
fn test_packet_clearing() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

        TestPacketClearing::default().run_test(&setup).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

#[test]
fn test_misbehaviour_detection() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

        TestMisbehaviourDetection::default()
            .run_test(&setup)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

// TODO: The current Gaia used, v18, uses a legacy proposals
#[cfg(feature = "ibc-go-v8")]
#[test]
fn test_upgrade_client() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

        hermes_ibc_test_suite::tests::upgrade_client::TestUpgradeClient::default()
            .run_test(&setup)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

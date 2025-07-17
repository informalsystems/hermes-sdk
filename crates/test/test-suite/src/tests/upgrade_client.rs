use core::marker::PhantomData;

use cgp::core::field::Index;
use hermes_prelude::*;
use hermes_test_components::test_case::traits::test_case::TestCase;
use hermes_test_components::test_case::traits::upgrade_client::{
    CanHandleUpgradeClient, CanSetupUpgradeClientTest,
};

use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestUpgradeClient<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestUpgradeClient<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestUpgradeClient<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B>
        + CanSetupUpgradeClientTest<Driver::ChainDriverA, Driver::ChainA, Driver::ChainB>
        + CanHandleUpgradeClient<Driver::ChainDriverA, Driver::ChainA, Driver::ChainB>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let setup_result = driver.setup_upgrade_client_test().await?;
        driver.handle_upgrade_client(&setup_result).await
    }
}

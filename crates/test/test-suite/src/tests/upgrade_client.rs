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
    Driver: CanUseBinaryTestDriverMethods<A, B>,
    Driver::ChainDriverA: CanSetupUpgradeClientTest<Driver::ChainDriverB>
        + CanHandleUpgradeClient<Driver::ChainDriverB>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_a();
        let chain_driver_b = driver.chain_driver_b();
        let client_id_b = driver.client_id_b();
        let setup_result = chain_driver_a
            .setup_upgrade_client_test(chain_driver_b, client_id_b)
            .await
            .map_err(Driver::raise_error)?;
        chain_driver_a
            .handle_upgrade_client(&setup_result, chain_driver_b, client_id_b)
            .await
            .map_err(Driver::raise_error)?;

        Ok(())
    }
}

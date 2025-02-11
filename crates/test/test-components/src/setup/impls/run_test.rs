use cgp::prelude::*;

use crate::setup::traits::driver::CanBuildTestDriver;
use crate::setup::traits::run_test::{TestRunner, TestRunnerComponent};
use crate::test_case::traits::test_case::TestCase;

/**
   Implementation for building a test driver and running one test with it.
*/
pub struct BuildDriverAndRunTest;

#[cgp_provider(TestRunnerComponent)]
impl<Setup, Driver, Test> TestRunner<Setup, Test> for BuildDriverAndRunTest
where
    Setup: CanBuildTestDriver<TestDriver = Driver> + CanRaiseAsyncError<Driver::Error>,
    Driver: Async + HasAsyncErrorType,
    Test: TestCase<Driver>,
{
    async fn run_test(setup: &Setup, test: &Test) -> Result<(), Setup::Error> {
        let driver = setup.build_driver().await?;

        test.run_test(&driver).await.map_err(Setup::raise_error)?;

        Ok(())
    }
}

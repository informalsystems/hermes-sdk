use cgp_core::prelude::*;

#[derive_component(TestRunnerComponent, TestRunner<Setup>)]
#[async_trait]
pub trait CanRunTest<Test>: HasErrorType
where
    Test: Async,
{
    async fn run_test(&self, test_case: &Test) -> Result<(), Self::Error>;
}

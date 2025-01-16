use cgp::prelude::*;

#[cgp_component {
  provider: TestRunner,
  context: Setup,
}]
#[async_trait]
pub trait CanRunTest<Test>: Async + HasAsyncErrorType
where
    Test: Async,
{
    async fn run_test(&self, test: &Test) -> Result<(), Self::Error>;
}

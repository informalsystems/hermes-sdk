use hermes_prelude::*;

#[async_trait]
pub trait TestCase<Driver>: Async
where
    Driver: HasAsyncErrorType,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error>;
}

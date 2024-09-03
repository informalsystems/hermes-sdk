use cgp::prelude::*;

#[async_trait]
pub trait TestCase<Driver>: Async
where
    Driver: HasErrorType,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error>;
}

use alloc::boxed::Box;

use cgp_core::prelude::*;

#[async_trait]
pub trait TestCase<TestContext>: Async
where
    TestContext: HasErrorType,
{
    async fn run_test(&self, test_context: &TestContext) -> Result<(), TestContext::Error>;
}

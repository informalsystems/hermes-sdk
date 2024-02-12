use cgp_core::async_trait;

use crate::output::Output;
use crate::Result;

#[async_trait]
pub trait CommandRunner<Context> {
    async fn run(&self, context: &Context) -> Result<Output>;
}

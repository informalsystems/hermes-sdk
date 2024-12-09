use cgp::prelude::*;

use crate::output::Output;
use crate::Result;

#[async_trait]
pub trait CommandRunner<Context> {
    async fn run(&self, context: &Context) -> Result<Output>;
}

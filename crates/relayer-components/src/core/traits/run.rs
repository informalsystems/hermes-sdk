use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use crate::std_prelude::*;

#[derive_component(RunnerComponent, Runner<App>)]
#[async_trait]
pub trait CanRun: HasErrorType {
    async fn run(&self) -> Result<(), Self::Error>;
}

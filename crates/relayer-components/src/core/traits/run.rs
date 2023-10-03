use cgp_core::{async_trait, derive_component, HasErrorType};

use crate::std_prelude::*;

#[derive_component(RunnerComponent, Runner<App>)]
#[async_trait]
pub trait CanRun: HasErrorType {
    async fn run(&self) -> Result<(), Self::Error>;
}

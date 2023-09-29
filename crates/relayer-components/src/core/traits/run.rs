use cgp_async::async_generic_trait;
use cgp_core::traits::HasErrorType;
use cgp_macros::derive_component;

use crate::std_prelude::*;

#[derive_component(RunnerComponent, Runner<App>)]
#[async_generic_trait]
pub trait CanRun: HasErrorType {
    async fn run(&self) -> Result<(), Self::Error>;
}

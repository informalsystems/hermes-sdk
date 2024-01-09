use cgp_core::prelude::*;

pub trait HasDriverType: Async {
    type Driver: Async;
}

#[derive_component(DriverBuilderComponent, DriverBuilder<Context>)]
#[async_trait]
pub trait CanBuildDriver: HasDriverType + HasErrorType {
    async fn build_driver(&self) -> Result<Self::Driver, Self::Error>;
}

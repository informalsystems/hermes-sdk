use cgp_core::prelude::*;

#[derive_component(DriverTypeComponent, ProvideDriverType<Setup>)]
pub trait HasDriverType: Async {
    type Driver: Async;
}

#[derive_component(DriverBuilderComponent, DriverBuilder<Context>)]
#[async_trait]
pub trait CanBuildDriver: HasDriverType + HasErrorType {
    async fn build_driver(&self) -> Result<Self::Driver, Self::Error>;
}

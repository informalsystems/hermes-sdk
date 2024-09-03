use cgp::prelude::*;

#[derive_component(DriverTypeComponent, ProvideTestDriverType<Setup>)]
pub trait HasTestDriverType: Async {
    type TestDriver: Async;
}

#[derive_component(DriverBuilderComponent, DriverBuilder<Context>)]
#[async_trait]
pub trait CanBuildTestDriver: HasTestDriverType + HasErrorType {
    async fn build_driver(&self) -> Result<Self::TestDriver, Self::Error>;
}

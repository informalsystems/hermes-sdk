use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_type {
    provider: TestDriverTypeProvider,
}]
pub trait HasTestDriverType: Async {
    type TestDriver: Async;
}

#[cgp_component {
    provider: DriverBuilder,
}]
#[async_trait]
pub trait CanBuildTestDriver: HasTestDriverType + HasAsyncErrorType {
    async fn build_driver(&self) -> Result<Self::TestDriver, Self::Error>;
}

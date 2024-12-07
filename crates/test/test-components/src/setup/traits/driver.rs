use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: TestDriverTypeComponent,
  provider: ProvideTestDriverType,
  context: Setup,
}]
pub trait HasTestDriverType: Async {
    type TestDriver: Async;
}

#[cgp_component {
  name: DriverBuilderComponent,
  provider: DriverBuilder,
}]
#[async_trait]
pub trait CanBuildTestDriver: HasTestDriverType + HasErrorType {
    async fn build_driver(&self) -> Result<Self::TestDriver, Self::Error>;
}

impl<Setup, Provider, TestDriver> ProvideTestDriverType<Setup> for WithProvider<Provider>
where
    Provider: ProvideType<Setup, TestDriverTypeComponent, Type = TestDriver>,
    Setup: Async,
    TestDriver: Async,
{
    type TestDriver = TestDriver;
}

use hermes_prelude::*;

use crate::traits::HasConfigType;

#[cgp_component {
  provider: ConfigWriter,
  context: App,
}]
#[async_trait]
pub trait CanWriteConfig: HasConfigType + HasAsyncErrorType {
    async fn write_config(&self, config: &Self::Config) -> Result<(), Self::Error>;
}

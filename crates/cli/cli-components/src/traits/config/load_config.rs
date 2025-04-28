use hermes_prelude::*;

use crate::traits::HasConfigType;

#[cgp_component {
  provider: ConfigLoader,
  context: App,
}]
#[async_trait]
pub trait CanLoadConfig: HasConfigType + HasAsyncErrorType {
    async fn load_config(&self) -> Result<Self::Config, Self::Error>;
}

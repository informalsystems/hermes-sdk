use cgp::prelude::*;

use crate::traits::types::config::HasConfigType;

#[cgp_component {
  provider: ConfigLoader,
  context: App,
}]
#[async_trait]
pub trait CanLoadConfig: HasConfigType + HasAsyncErrorType {
    async fn load_config(&self) -> Result<Self::Config, Self::Error>;
}

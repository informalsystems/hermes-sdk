use cgp::prelude::*;

use crate::traits::types::config::HasConfigType;

#[cgp_component {
  name: ConfigLoaderComponent,
  provider: ConfigLoader,
  context: App,
}]
#[async_trait]
pub trait CanLoadConfig: HasConfigType + HasErrorType {
    async fn load_config(&self) -> Result<Self::Config, Self::Error>;
}

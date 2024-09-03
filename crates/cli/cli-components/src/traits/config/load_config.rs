use cgp::prelude::*;

use crate::traits::types::config::HasConfigType;

#[derive_component(ConfigLoaderComponent, ConfigLoader<App>)]
#[async_trait]
pub trait CanLoadConfig: HasConfigType + HasErrorType {
    async fn load_config(&self) -> Result<Self::Config, Self::Error>;
}

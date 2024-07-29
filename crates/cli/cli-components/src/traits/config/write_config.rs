use cgp_core::prelude::*;

use crate::traits::types::config::HasConfigType;

#[derive_component(ConfigWriterComponent, ConfigWriter<App>)]
#[async_trait]
pub trait CanWriteConfig: HasConfigType + HasErrorType {
    async fn write_config(&self, config: &Self::Config) -> Result<(), Self::Error>;
}

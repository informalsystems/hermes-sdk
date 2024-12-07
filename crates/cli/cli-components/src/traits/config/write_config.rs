use cgp::prelude::*;

use crate::traits::types::config::HasConfigType;

#[cgp_component {
  name: ConfigWriterComponent,
  provider: ConfigWriter,
  context: App,
}]
#[async_trait]
pub trait CanWriteConfig: HasConfigType + HasErrorType {
    async fn write_config(&self, config: &Self::Config) -> Result<(), Self::Error>;
}

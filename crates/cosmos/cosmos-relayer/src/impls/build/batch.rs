use hermes_relayer_components_extra::batch::traits::config::HasBatchConfig;
use hermes_relayer_components_extra::batch::types::config::BatchConfig;

use crate::contexts::builder::CosmosBuilder;

impl HasBatchConfig for CosmosBuilder {
    fn batch_config(&self) -> &BatchConfig {
        &self.batch_config
    }
}

use cgp_core::Async;

use crate::batch::types::config::BatchConfig;

pub trait HasBatchConfig: Async {
    fn batch_config(&self) -> &BatchConfig;
}

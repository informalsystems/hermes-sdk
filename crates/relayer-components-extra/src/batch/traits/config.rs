use cgp_core::traits::Async;

use crate::batch::types::config::BatchConfig;

pub trait HasBatchConfig: Async {
    fn batch_config(&self) -> &BatchConfig;
}

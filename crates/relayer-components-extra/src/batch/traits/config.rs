use cgp_core::traits::sync::Async;

use crate::batch::types::config::BatchConfig;

pub trait HasBatchConfig: Async {
    fn batch_config(&self) -> &BatchConfig;
}

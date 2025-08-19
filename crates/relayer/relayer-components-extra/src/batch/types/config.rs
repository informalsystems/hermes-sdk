use core::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    pub max_message_count: usize,
    pub max_tx_size: usize,
    pub buffer_size: usize,
    pub max_delay: Duration,
    pub sleep_time: Duration,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_message_count: 300,
            max_tx_size: 1000000,
            buffer_size: 1000000,
            max_delay: Duration::from_secs(30),
            sleep_time: Duration::from_millis(100),
        }
    }
}

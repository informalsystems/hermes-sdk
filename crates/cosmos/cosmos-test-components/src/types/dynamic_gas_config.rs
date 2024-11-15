#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DynamicGasConfig {
    pub multiplier: f64,
    pub max: f64,
}

impl Default for DynamicGasConfig {
    fn default() -> Self {
        Self {
            multiplier: 1.1,
            max: 1.6,
        }
    }
}

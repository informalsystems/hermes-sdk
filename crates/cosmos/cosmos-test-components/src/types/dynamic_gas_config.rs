#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DynamicGasConfig {
    pub multiplier: f64,
    pub max: f64,
}

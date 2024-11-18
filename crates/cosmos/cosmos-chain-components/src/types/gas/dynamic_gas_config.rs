use crate::types::gas::eip_type::EipQueryType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DynamicGasConfig {
    pub enabled: bool,
    pub multiplier: f64,
    pub max: f64,
    pub eip_query_type: EipQueryType,
    pub denom: String,
}

impl DynamicGasConfig {
    pub fn new(
        enabled: bool,
        multiplier: f64,
        max: f64,
        raw_eip_query_type: &str,
        denom: &str,
    ) -> Self {
        let eip_query_type = raw_eip_query_type
            .parse::<EipQueryType>()
            .unwrap_or_default();
        Self {
            enabled,
            multiplier,
            max,
            eip_query_type,
            denom: denom.to_owned(),
        }
    }
}

impl Default for DynamicGasConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            multiplier: 1.1,
            max: 1.6,
            eip_query_type: Default::default(),
            denom: "stake".to_owned(),
        }
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SovereignEvent {
    pub event_value: serde_json::Value,
    pub module_name: String,
    pub module_address: String,
}

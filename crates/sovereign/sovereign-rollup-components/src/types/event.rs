use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SovereignEvent {
    pub event_key: String,
    pub event_value: serde_json::Value,
    pub module_name: String,
}

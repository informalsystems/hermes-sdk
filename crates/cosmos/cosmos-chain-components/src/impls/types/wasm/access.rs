use crate::impls::types::wasm::cosmwasm::AccessConfig;

#[derive(Clone, Debug)]
pub enum WasmAccessConfig {
    Unspecified,
    Nobody,
    Everybody,
    AnyOfAddresses(Vec<String>),
}

impl From<WasmAccessConfig> for AccessConfig {
    fn from(value: WasmAccessConfig) -> Self {
        match value {
            WasmAccessConfig::Unspecified => AccessConfig {
                permission: 0,
                addresses: vec![],
            },
            WasmAccessConfig::Nobody => AccessConfig {
                permission: 1,
                addresses: vec![],
            },
            WasmAccessConfig::Everybody => AccessConfig {
                permission: 3,
                addresses: vec![],
            },
            WasmAccessConfig::AnyOfAddresses(addresses) => AccessConfig {
                permission: 4,
                addresses,
            },
        }
    }
}

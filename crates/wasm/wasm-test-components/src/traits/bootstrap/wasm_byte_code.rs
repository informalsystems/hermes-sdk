use hermes_prelude::*;

#[cgp_getter {
    provider: WasmAdditionalByteCodeGetter,
    context: Bootstrap,
}]
pub trait HasWasmAdditionalByteCode {
    fn wasm_additional_byte_codes(&self) -> &Vec<Vec<u8>>;
}

use hermes_prelude::*;

#[cgp_getter {
    provider: WasmClientByteCodeGetter,
    context: Bootstrap,
}]
pub trait HasWasmClientByteCode {
    fn wasm_client_byte_code(&self) -> &Vec<u8>;
}

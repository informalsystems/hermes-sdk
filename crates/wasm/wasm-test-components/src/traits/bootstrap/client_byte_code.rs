use cgp::prelude::*;

#[derive_component(WasmClientByteCodeGetterComponent, WasmClientByteCodeGetter<Bootstrap>)]
pub trait HasWasmClientByteCode {
    fn wasm_client_byte_code(&self) -> &Vec<u8>;
}

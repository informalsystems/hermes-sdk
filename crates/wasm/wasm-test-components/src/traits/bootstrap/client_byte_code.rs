use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;

#[cgp_component {
  provider: WasmClientByteCodeGetter,
  context: Bootstrap,
}]
pub trait HasWasmClientByteCode {
    fn wasm_client_byte_code(&self) -> &Vec<u8>;
}

impl<Bootstrap> WasmClientByteCodeGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("wasm_client_byte_code"), Value = Vec<u8>>,
{
    fn wasm_client_byte_code(bootstrap: &Bootstrap) -> &Vec<u8> {
        bootstrap.get_field(PhantomData)
    }
}

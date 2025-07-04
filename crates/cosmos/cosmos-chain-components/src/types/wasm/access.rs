use hermes_prelude::*;

#[cgp_type]
pub trait HasWasmAccessType: Sized + Async {
    type WasmAccess: Async;
}

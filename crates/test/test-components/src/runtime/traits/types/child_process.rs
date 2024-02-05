use cgp_core::prelude::*;

#[derive_component(ChildProcessTypeComponent, ProvideChildProcessType<Runtime>)]
pub trait HasChildProcessType: Async {
    type ChildProcess: Async;
}

pub type ChildProcessOf<Runtime> = <Runtime as HasChildProcessType>::ChildProcess;

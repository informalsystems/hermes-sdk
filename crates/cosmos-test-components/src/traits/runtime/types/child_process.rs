use cgp_core::prelude::*;

pub trait HasChildProcessType: Async {
    type ChildProcess: Async;
}

pub type ChildProcess<Runtime> = <Runtime as HasChildProcessType>::ChildProcess;

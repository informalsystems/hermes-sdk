use cgp_core::prelude::*;

pub trait HasChildProcessType: Async {
    type ChildProcess: Async;
}

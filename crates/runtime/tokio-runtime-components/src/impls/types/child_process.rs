use cgp_core::prelude::*;
use hermes_runtime_components::traits::os::child_process::ProvideChildProcessType;
use tokio::process::Child;

pub struct ProvideTokioChildProcessType;

impl<Runtime> ProvideChildProcessType<Runtime> for ProvideTokioChildProcessType
where
    Runtime: Async,
{
    type ChildProcess = Child;
}

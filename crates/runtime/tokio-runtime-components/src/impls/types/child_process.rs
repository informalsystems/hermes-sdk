use cgp_core::prelude::*;
use hermes_test_components::runtime::traits::types::child_process::ProvideChildProcessType;
use tokio::process::Child;

pub struct ProvideTokioChildProcessType;

impl<Runtime> ProvideChildProcessType<Runtime> for ProvideTokioChildProcessType
where
    Runtime: Async,
{
    type ChildProcess = Child;
}

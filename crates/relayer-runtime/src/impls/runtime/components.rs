use cgp_core::prelude::*;
use ibc_test_components::runtime::traits::child_process::ChildProcessStarterComponent;
use ibc_test_components::runtime::traits::types::file_path::FilePathTypeComponent;

use crate::impls::runtime::child_process::StartTokioChildProcess;
use crate::impls::runtime::file_path::ProvideStdPathType;
use crate::types::runtime::TokioRuntimeContext;

pub struct TokioRuntimeComponents;

impl HasComponents for TokioRuntimeContext {
    type Components = TokioRuntimeComponents;
}

delegate_components!(
    TokioRuntimeComponents;
    FilePathTypeComponent: ProvideStdPathType,
    ChildProcessStarterComponent: StartTokioChildProcess,
);

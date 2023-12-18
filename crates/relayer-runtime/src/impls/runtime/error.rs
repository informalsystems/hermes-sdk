use alloc::sync::Arc;
use cgp_core::{ErrorRaiser, ProvideErrorType};
use core::str::Utf8Error;
use std::io::Error as IoError;
use tokio_runtime_components::impls::child_process::PrematureChildProcessExitError;
use tokio_runtime_components::impls::exec_command::ExecCommandFailure;

use crate::impls::runtime::components::RelayerRuntimeComponents;
use crate::types::error::TokioRuntimeError;
use crate::types::runtime::TokioRuntimeContext;

impl ProvideErrorType<TokioRuntimeContext> for RelayerRuntimeComponents {
    type Error = TokioRuntimeError;
}

impl ErrorRaiser<TokioRuntimeContext, PrematureChildProcessExitError> for RelayerRuntimeComponents {
    fn raise_error(e: PrematureChildProcessExitError) -> TokioRuntimeError {
        TokioRuntimeError::PrematureChildProcessExit {
            exit_status: e.exit_status,
            output: e.output,
        }
    }
}

impl ErrorRaiser<TokioRuntimeContext, IoError> for RelayerRuntimeComponents {
    fn raise_error(e: IoError) -> TokioRuntimeError {
        TokioRuntimeError::Io(Arc::new(e))
    }
}

impl ErrorRaiser<TokioRuntimeContext, Utf8Error> for RelayerRuntimeComponents {
    fn raise_error(e: Utf8Error) -> TokioRuntimeError {
        TokioRuntimeError::Utf8(e)
    }
}

impl ErrorRaiser<TokioRuntimeContext, ExecCommandFailure> for RelayerRuntimeComponents {
    fn raise_error(e: ExecCommandFailure) -> TokioRuntimeError {
        TokioRuntimeError::ExecCommandFailure {
            command: e.command,
            exit_code: e.exit_code,
            stdout: e.stdout,
            stderr: e.stderr,
        }
    }
}

use alloc::sync::Arc;
use cgp_core::{CanRaiseError, HasErrorType};
use core::str::Utf8Error;
use std::io::Error as IoError;
use tokio_runtime_components::impls::child_process::PrematureChildProcessExitError;
use tokio_runtime_components::impls::exec_command::ExecCommandFailure;

use crate::types::error::TokioRuntimeError;
use crate::types::runtime::TokioRuntimeContext;

impl HasErrorType for TokioRuntimeContext {
    type Error = TokioRuntimeError;
}

impl CanRaiseError<PrematureChildProcessExitError> for TokioRuntimeContext {
    fn raise_error(e: PrematureChildProcessExitError) -> TokioRuntimeError {
        TokioRuntimeError::PrematureChildProcessExit {
            exit_status: e.exit_status,
            output: e.output,
        }
    }
}

impl CanRaiseError<IoError> for TokioRuntimeContext {
    fn raise_error(e: IoError) -> TokioRuntimeError {
        TokioRuntimeError::Io(Arc::new(e))
    }
}

impl CanRaiseError<Utf8Error> for TokioRuntimeContext {
    fn raise_error(e: Utf8Error) -> TokioRuntimeError {
        TokioRuntimeError::Utf8(e)
    }
}

impl CanRaiseError<ExecCommandFailure> for TokioRuntimeContext {
    fn raise_error(e: ExecCommandFailure) -> TokioRuntimeError {
        TokioRuntimeError::ExecCommandFailure {
            command: e.command,
            exit_code: e.exit_code,
            stdout: e.stdout,
            stderr: e.stderr,
        }
    }
}

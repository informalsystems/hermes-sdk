use alloc::sync::Arc;
use core::str::Utf8Error;
use std::io::Error as IoError;

use cgp_core::error::{ErrorRaiser, ProvideErrorType};
use hermes_async_runtime_components::channel::types::ChannelClosedError;
use hermes_tokio_runtime_components::impls::os::child_process::PrematureChildProcessExitError;
use hermes_tokio_runtime_components::impls::os::exec_command::ExecCommandFailure;

use crate::impls::runtime::components::HermesRuntimeComponents;
use crate::types::error::TokioRuntimeError;
use crate::types::runtime::HermesRuntime;

impl ProvideErrorType<HermesRuntime> for HermesRuntimeComponents {
    type Error = TokioRuntimeError;
}

impl ErrorRaiser<HermesRuntime, PrematureChildProcessExitError> for HermesRuntimeComponents {
    fn raise_error(e: PrematureChildProcessExitError) -> TokioRuntimeError {
        TokioRuntimeError::PrematureChildProcessExit {
            exit_status: e.exit_status,
            stdout: e.stdout,
            stderr: e.stderr,
        }
    }
}

impl ErrorRaiser<HermesRuntime, IoError> for HermesRuntimeComponents {
    fn raise_error(e: IoError) -> TokioRuntimeError {
        TokioRuntimeError::Io(Arc::new(e))
    }
}

impl ErrorRaiser<HermesRuntime, Utf8Error> for HermesRuntimeComponents {
    fn raise_error(e: Utf8Error) -> TokioRuntimeError {
        TokioRuntimeError::Utf8(e)
    }
}

impl ErrorRaiser<HermesRuntime, ChannelClosedError> for HermesRuntimeComponents {
    fn raise_error(_e: ChannelClosedError) -> TokioRuntimeError {
        TokioRuntimeError::ChannelClosed
    }
}

impl ErrorRaiser<HermesRuntime, ExecCommandFailure> for HermesRuntimeComponents {
    fn raise_error(e: ExecCommandFailure) -> TokioRuntimeError {
        TokioRuntimeError::ExecCommandFailure {
            command: e.command,
            exit_code: e.exit_code,
            stdout: e.stdout,
            stderr: e.stderr,
        }
    }
}

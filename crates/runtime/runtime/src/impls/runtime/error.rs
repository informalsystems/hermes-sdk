use alloc::sync::Arc;
use core::str::Utf8Error;
use std::io::Error as IoError;
use std::process::ExitStatus;

use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent};
use cgp::prelude::{cgp_provider, *};
use hermes_async_runtime_components::channel::types::ErrChannelClosed;
use hermes_tokio_runtime_components::impls::os::child_process::PrematureChildProcessExitError;
use hermes_tokio_runtime_components::impls::os::exec_command::{
    CommandNotFound, ExecCommandFailure,
};

use crate::types::error::TokioRuntimeError;
use crate::types::runtime::{HermesRuntime, HermesRuntimeComponents};

#[cgp_provider(ErrorTypeProviderComponent)]
impl ErrorTypeProvider<HermesRuntime> for HermesRuntimeComponents {
    type Error = TokioRuntimeError;
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<HermesRuntime, PrematureChildProcessExitError> for HermesRuntimeComponents {
    fn raise_error(e: PrematureChildProcessExitError) -> TokioRuntimeError {
        TokioRuntimeError::PrematureChildProcessExit {
            exit_status: e.exit_status,
            stdout: e.stdout,
            stderr: e.stderr,
        }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<HermesRuntime, ExitStatus> for HermesRuntimeComponents {
    fn raise_error(exit_status: ExitStatus) -> TokioRuntimeError {
        TokioRuntimeError::ChildProcessExitFailure { exit_status }
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<HermesRuntime, IoError> for HermesRuntimeComponents {
    fn raise_error(e: IoError) -> TokioRuntimeError {
        TokioRuntimeError::Io(Arc::new(e))
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<HermesRuntime, Utf8Error> for HermesRuntimeComponents {
    fn raise_error(e: Utf8Error) -> TokioRuntimeError {
        TokioRuntimeError::Utf8(e)
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<HermesRuntime, ErrChannelClosed> for HermesRuntimeComponents {
    fn raise_error(_e: ErrChannelClosed) -> TokioRuntimeError {
        TokioRuntimeError::ChannelClosed
    }
}

#[cgp_provider(ErrorRaiserComponent)]
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

#[cgp_provider(ErrorRaiserComponent)]
impl ErrorRaiser<HermesRuntime, CommandNotFound> for HermesRuntimeComponents {
    fn raise_error(e: CommandNotFound) -> TokioRuntimeError {
        TokioRuntimeError::CommandNotFound { command: e.command }
    }
}

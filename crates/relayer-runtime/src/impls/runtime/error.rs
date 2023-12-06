use cgp_core::{CanRaiseError, HasErrorType};

use crate::impls::runtime::child_process::PrematureChildProcessExitError;
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

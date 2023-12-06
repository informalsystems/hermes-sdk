use cgp_core::HasErrorType;

use crate::types::error::TokioRuntimeError;
use crate::types::runtime::TokioRuntimeContext;

impl HasErrorType for TokioRuntimeContext {
    type Error = TokioRuntimeError;
}

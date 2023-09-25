use cgp_core::traits::error::HasErrorType;

use crate::types::error::Error;
use crate::types::runtime::TokioRuntimeContext;

impl HasErrorType for TokioRuntimeContext {
    type Error = Error;
}

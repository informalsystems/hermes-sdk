use alloc::sync::Arc;
use core::fmt::{self, Debug, Formatter};

use eyre::Report;

#[derive(Clone)]
pub struct Error {
    pub is_retryable: bool,
    pub detail: ErrorDetail,
}

#[derive(Clone)]
pub enum ErrorDetail {
    Report(Arc<Report>),
    Wrapped(String, Arc<ErrorDetail>),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_retryable {
            write!(f, "retryable error: ")?;
        }

        Debug::fmt(&self.detail, f)
    }
}

impl Debug for ErrorDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorDetail::Report(report) => Debug::fmt(report, f),
            ErrorDetail::Wrapped(message, detail) => {
                write!(f, "{}: {:?}", message, detail)
            }
        }
    }
}

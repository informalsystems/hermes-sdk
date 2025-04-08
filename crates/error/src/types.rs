use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::fmt::{self, Debug, Display, Formatter};

use eyre::{eyre, Report};

pub type HermesError = Error;

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

impl<E> From<E> for Error
where
    Report: From<E>,
{
    fn from(e: E) -> Self {
        Self::report(e)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Report> for Error {
    fn into(self) -> Report {
        eyre!("{}", self)
    }
}

impl Error {
    pub fn report<E>(e: E) -> Self
    where
        Report: From<E>,
    {
        Self {
            is_retryable: false,
            detail: ErrorDetail::Report(Arc::new(e.into())),
        }
    }

    pub fn wrap<M>(self, message: M) -> Self
    where
        M: Display,
    {
        Self {
            is_retryable: self.is_retryable,
            detail: ErrorDetail::Wrapped(message.to_string(), Arc::new(self.detail)),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_retryable {
            write!(f, "retryable error: ")?;
        }

        Debug::fmt(&self.detail, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.detail.fmt(f)
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

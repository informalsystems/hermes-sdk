use alloc::sync::Arc;
use core::fmt::Display;
use core::fmt::{self, Debug, Formatter};

use cgp_core::ErrorRaiser;
use cgp_core::HasErrorType;
use eyre::{eyre, Report};

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

pub struct ReturnError;

impl<Context> ErrorRaiser<Context, Error> for ReturnError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}

pub struct MessageError<const RETRYABLE: bool>;

pub type MessageRetryableError = MessageError<true>;
pub type MessageNonRetryableError = MessageError<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for MessageError<RETRYABLE>
where
    Context: HasErrorType<Error = Error>,
    E: Display,
{
    fn raise_error(e: E) -> Error {
        Error {
            is_retryable: RETRYABLE,
            detail: ErrorDetail::Report(Arc::new(eyre!("{}", e))),
        }
    }
}

pub struct ReportError<const RETRYABLE: bool>;

pub type ReportRetryableError = ReportError<true>;
pub type ReportNonRetryableError = ReportError<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for ReportError<RETRYABLE>
where
    Context: HasErrorType<Error = Error>,
    Report: From<E>,
{
    fn raise_error(e: E) -> Error {
        Error {
            is_retryable: RETRYABLE,
            detail: ErrorDetail::Report(Arc::new(e.into())),
        }
    }
}

pub struct WrapErrorDetail;

impl<Context, Detail> ErrorRaiser<Context, (Detail, Error)> for WrapErrorDetail
where
    Context: HasErrorType<Error = Error>,
    Detail: Debug,
{
    fn raise_error((detail, e): (Detail, Error)) -> Error {
        Error {
            is_retryable: e.is_retryable,
            detail: ErrorDetail::Wrapped(format!("{:?}", detail), Arc::new(e.detail)),
        }
    }
}

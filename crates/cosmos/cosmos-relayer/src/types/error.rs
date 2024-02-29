use alloc::sync::Arc;
use core::fmt::Display;
use core::fmt::{self, Debug, Formatter};
use std::error::Error as StdError;

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

impl<E> From<E> for Error
where
    Report: From<E>,
{
    fn from(e: E) -> Self {
        Self::report(e)
    }
}

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

pub trait ErrorWrapper {
    type Value;

    fn wrap_error<M>(self, message: M) -> Result<Self::Value, Error>
    where
        M: Display;
}

impl<T, E> ErrorWrapper for Result<T, E>
where
    Error: From<E>,
{
    type Value = T;

    fn wrap_error<M>(self, message: M) -> Result<Self::Value, Error>
    where
        M: Display,
    {
        self.map_err(|e| Error::from(e).wrap(message))
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
        Debug::fmt(self, f)
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

pub struct DebugError<const RETRYABLE: bool>;

pub type DebugRetryableError = DebugError<true>;
pub type DebugNonRetryableError = DebugError<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for DebugError<RETRYABLE>
where
    Context: HasErrorType<Error = Error>,
    E: Debug,
{
    fn raise_error(e: E) -> Error {
        Error {
            is_retryable: RETRYABLE,
            detail: ErrorDetail::Report(Arc::new(eyre!("{:?}", e))),
        }
    }
}

pub struct DisplayError<const RETRYABLE: bool>;

pub type DisplayRetryableError = DisplayError<true>;
pub type DisplayNonRetryableError = DisplayError<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for DisplayError<RETRYABLE>
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

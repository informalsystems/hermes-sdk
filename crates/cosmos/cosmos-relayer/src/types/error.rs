use alloc::sync::Arc;
use core::convert::Infallible;
use core::fmt::{self, Debug, Display, Formatter};

use cgp_core::error::{ErrorRaiser, HasErrorType, ProvideErrorType};
use cgp_core::Async;
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

pub struct ProvideCosmosError;

impl<Context> ProvideErrorType<Context> for ProvideCosmosError
where
    Context: Async,
{
    type Error = Error;
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

pub struct DebugErrorWithRetry<const RETRYABLE: bool>;

pub type DebugRetryableError = DebugErrorWithRetry<true>;
pub type DebugError = DebugErrorWithRetry<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for DebugErrorWithRetry<RETRYABLE>
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

pub struct DisplayErrorWithRetry<const RETRYABLE: bool>;

pub type DisplayRetryableError = DisplayErrorWithRetry<true>;
pub type DisplayError = DisplayErrorWithRetry<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for DisplayErrorWithRetry<RETRYABLE>
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

pub struct ReportErrorWithRetry<const RETRYABLE: bool>;

pub type ReportRetryableError = ReportErrorWithRetry<true>;
pub type ReportError = ReportErrorWithRetry<false>;

impl<Context, E, const RETRYABLE: bool> ErrorRaiser<Context, E> for ReportErrorWithRetry<RETRYABLE>
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

pub struct HandleInfallible;

impl<Context> ErrorRaiser<Context, Infallible> for HandleInfallible
where
    Context: HasErrorType,
{
    fn raise_error(e: Infallible) -> Context::Error {
        match e {}
    }
}

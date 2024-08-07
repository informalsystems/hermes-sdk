//! Custom-made solution to output a JSON return message and ensure a return code
//! from a CLI command. The main use-case for this module is to provide a consistent output for
//! queries and transactions.
//!
//! The examples below rely on crate-private methods (for this reason, doctests are ignored).
//! They are intended for contributors to crate `relayer-cli`, and _not_ for users of this binary.
//!
//! ## Examples on how to use the quick-access constructors:
//!
//! - Exit from a query/tx with a `String` error:
//!
//! ```ignore
//! let e = String::from("error message");
//! Output::error(e).exit();
//! // or as an alternative:
//! Output::error(json!("error occurred")).exit();
//! ```
//!
//! - Exit from a query/tx with an error of type `anomaly`:
//!   In the case where the error is a complex type such as anomaly (including backtraces), it is
//!   better to simplify the output and only write out the chain of error sources, which we can
//!   achieve with `format!("{}", e)`. The complete solution is as follows:
//!
//! ```ignore
//! let e: Error = Kind::Query.into();
//! Output::error(format!("{}", e)).exit();
//! ```
//!
//! #### Note:
//! The resulting output that this approach generates is determined by the 'error' annotation given
//! to the error object `Kind::Query`. If this error object comprises any positional arguments,
//! e.g. as achieved by `Query(String, String)`, then it is important to cover these arguments
//! in the `error` annotation, for instance:
//! ```ignore
//! #[derive(Debug, Error)]
//! pub enum Kind {
//!     #[error("failed with underlying causes: {0}, {1}")]
//!     Query(String, String),
//!     // ...
//! }
//! ```
//!
//! - Exit from a query/tx with success:
//!
//! ```ignore
//! let cs = ChannelEnd::default();
//! Output::success(cs).exit();
//! ```
//!
//! - Exit from a query/tx with success and multiple objects in the result:
//!
//! ```ignore
//! let h = Height::default();
//! let end = ConnectionEnd::default();
//! Output::success(h).with_result(end).exit();
//! ```

use core::fmt::{self, Debug};
use std::sync::OnceLock;

use serde::Serialize;
use tracing::{error, info};

static JSON: OnceLock<bool> = OnceLock::new();

/// Functional-style method to exit a program.
///
/// ## Note: See `Output::exit()` for the preferred method of exiting a relayer command.
pub fn exit_with(out: Output) -> ! {
    let status = out.status;

    out.print();

    // The return code
    if status == Status::Error {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}

/// Return whether or not JSON output is enabled.
pub fn json() -> bool {
    *JSON.get_or_init(|| false)
}

/// Set whether or not JSON output is enabled.
pub fn set_json(enabled: bool) {
    JSON.set(enabled).expect("failed to set JSON mode")
}

/// Exits the program. Useful when a type produces an error which can no longer be propagated, and
/// the program must exit instead.
///
/// ## Example of use
/// - Without this function:
/// ```ignore
/// let res = ForeignClient::new(chains.src.clone(), chains.dst.clone());
/// let client = match res {
///     Ok(client) => client,
///     Err(e) => Output::error(format!("{}", e)).exit(),
/// };
/// ```
/// - With support from `exit_with_unrecoverable_error`:
/// ```ignore
/// let client_a = ForeignClient::new(chains.src.clone(), chains.dst.clone())
///     .unwrap_or_else(exit_with_unrecoverable_error);
/// ```
pub fn exit_with_unrecoverable_error<T, E: fmt::Display>(err: E) -> T {
    Output::error(format!("{err}")).exit()
}

/// The result to display before quitting, can either be a JSON value, some plain text,
/// a value to print with its Debug instance, or nothing.
#[derive(Debug)]
pub enum Result {
    Json(serde_json::Value),
    Text(String),
    Value(Box<dyn fmt::Debug + Send + Sync + 'static>),
    Nothing,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Result::Json(v) => write!(f, "{}", serde_json::to_string_pretty(v).unwrap()),
            Result::Text(t) => write!(f, "{t}"),
            Result::Value(v) => write!(f, "{v:#?}"),
            Result::Nothing => Ok(()),
        }
    }
}

/// A CLI output with support for JSON serialization. The only mandatory field is the `status`,
/// which typically signals a success (UNIX process return code `0`) or an error (code `1`). An
/// optional `result` can be added to an output.
///
pub struct Output {
    /// The return status
    pub status: Status,

    /// The result of a command, such as the output from a query or transaction.
    pub result: Result,
}

impl Output {
    /// Constructs a new `Output` with the provided `status` and an empty `result`.
    pub fn new(status: Status) -> Self {
        Output {
            status,
            result: Result::Nothing,
        }
    }

    /// Constructor that returns a new `Output` having a `Success` status and empty `result`.
    pub fn with_success() -> Self {
        Output::new(Status::Success)
    }

    /// Constructor that returns a new `Output` having an `Error` status and empty `result`.
    pub fn with_error() -> Self {
        Output::new(Status::Error)
    }

    /// Builder-style method for attaching a result to an output object.
    pub fn with_result<R>(mut self, result: R) -> Self
    where
        R: Serialize + Debug + Send + Sync + 'static,
    {
        if json() {
            self.result = Result::Json(serde_json::to_value(result).unwrap());
        } else {
            self.result = Result::Value(Box::new(result));
        }

        self
    }

    /// Builder-style method for attaching a plain text message to an output object.
    pub fn with_msg(mut self, msg: impl ToString) -> Self {
        self.result = Result::Text(msg.to_string());
        self
    }

    /// Quick-access constructor for an output signalling a success `status` and tagged with the
    /// input `result`.
    pub fn success<R>(result: R) -> Self
    where
        R: Serialize + Debug + Send + Sync + 'static,
    {
        Output::with_success().with_result(result)
    }

    /// Quick-access constructor for an output message signalling a error `status`.
    pub fn error(msg: impl ToString) -> Self {
        Output::with_error().with_msg(msg)
    }

    /// Quick-access constructor for an output signalling a success `status` and tagged with the
    /// input `result`.
    pub fn success_msg(msg: impl ToString) -> Self {
        Output::with_success().with_msg(msg)
    }

    pub fn print(self) {
        if json() {
            println!("{}", serde_json::to_string(&self.into_json()).unwrap());
        } else {
            match self.status {
                Status::Success => info!("{}", self.result),
                Status::Error => error!("{}", self.result),
            }
        }
    }

    /// Exits from the process with the current output. Convenience wrapper over `exit_with`.
    pub fn exit(self) -> ! {
        exit_with(self);
    }

    /// Convert this output value to a JSON value
    pub fn into_json(self) -> serde_json::Value {
        let result = match self.result {
            Result::Json(v) => v,
            Result::Text(v) => serde_json::Value::String(v),
            Result::Value(v) => serde_json::Value::String(format!("{v:#?}")),
            Result::Nothing => serde_json::Value::String("no output".to_string()),
        };

        serde_json::json!({
            "status": self.status,
            "result": result,
        })
    }
}

/// Represents the exit status of any CLI command
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Status {
    #[serde(rename(serialize = "success"))]
    Success,

    #[serde(rename(serialize = "error"))]
    Error,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Success => write!(f, "Success"),
            Status::Error => write!(f, "Error"),
        }
    }
}

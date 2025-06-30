use alloc::sync::Arc;
use core::fmt::{Debug, Display};
use core::str::Utf8Error;
use std::error::Error;
use std::io::Error as IoError;
use std::process::ExitStatus;

#[derive(Clone)]
pub enum TokioRuntimeError {
    ChannelClosed,
    PoisonedLock,
    Io(Arc<IoError>),
    Utf8(Utf8Error),
    PrematureChildProcessExit {
        exit_status: ExitStatus,
        stdout: String,
        stderr: String,
    },
    ChildProcessExitFailure {
        exit_status: ExitStatus,
    },
    ExecCommandFailure {
        command: String,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
    },
    CommandNotFound {
        command: String,
    },
}

impl Display for TokioRuntimeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ChannelClosed => {
                write!(f, "unexpected closure of internal rust channels")?;
            }
            Self::PoisonedLock => {
                write!(f, "poisoned mutex lock")?;
            }
            Self::Io(e) => {
                write!(f, "{e}")?;
            }
            Self::Utf8(e) => {
                write!(f, "{e}")?;
            }
            Self::PrematureChildProcessExit {
                exit_status,
                stderr,
                ..
            } => {
                write!(f, "expected child process to be running, but it exited immediately with exit status {exit_status} and stderr: {stderr}")?;
            }
            Self::ExecCommandFailure {
                command,
                exit_code,
                stderr,
                ..
            } => {
                write!(
                    f,
                    "execution of command {command} failed with exit code {exit_code:?}. stderr: {stderr}"
                )?;
            }
            Self::ChildProcessExitFailure { exit_status } => {
                write!(
                    f,
                    "child process exited with non-success status {exit_status}"
                )?;
            }
            Self::CommandNotFound { command } => {
                write!(
                    f,
                    "failed to execute command due to command not found: {command}"
                )?;
            }
        };

        Ok(())
    }
}

impl Debug for TokioRuntimeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self}")
    }
}

impl Error for TokioRuntimeError {}

use alloc::sync::Arc;
use core::fmt::Display;
use core::str::Utf8Error;
use std::error::Error;
use std::io::Error as IoError;
use std::process::ExitStatus;

#[derive(Clone, Debug)]
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
    ExecCommandFailure {
        command: String,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
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
                write!(f, "expected child process to be running, but it exited immediately with exit status {} and stderr: {}", exit_status, stderr)?;
            }
            Self::ExecCommandFailure {
                command,
                exit_code,
                stderr,
                ..
            } => {
                write!(
                    f,
                    "execution of command {} failed with exit code {:?}. stderr: {}",
                    command, exit_code, stderr
                )?;
            }
        };

        Ok(())
    }
}

impl Error for TokioRuntimeError {}

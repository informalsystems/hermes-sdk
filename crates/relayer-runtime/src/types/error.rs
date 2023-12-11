use core::fmt::Display;
use std::error::Error;
use std::process::ExitStatus;

#[derive(Clone, Debug)]
pub enum TokioRuntimeError {
    ChannelClosed,
    PoisonedLock,
    PrematureChildProcessExit {
        exit_status: ExitStatus,
        output: String,
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
            Self::PrematureChildProcessExit {
                exit_status,
                output,
            } => {
                write!(f, "expected child process to be running, but it exited immediately with exit status {} and output: {}", exit_status, output)?;
            }
        };

        Ok(())
    }
}

impl Error for TokioRuntimeError {}

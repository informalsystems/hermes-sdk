use std::process::ExitStatus;

use flex_error::define_error;

define_error! {
    #[derive(Clone, Debug)]
    Error {
        ChannelClosed
            | _ | { "unexpected closure of internal rust channels" },

        PoisonedLock
            | _ | { "poisoned mutex lock" },

        PrematureChildProcessExit
            {
                exit_status: ExitStatus,
                output: String,
            }
            | e | {
                format_args!("expected child process to be running, but it exited immediately with exit status {} and output: {}", e.exit_status, e.output)
            },
    }
}

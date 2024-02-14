#![recursion_limit = "256"]

use hermes_cli::application::HermesCli;
use hermes_cli_framework::application::boot;

fn main() {
    if let Err(e) = boot::<HermesCli>() {
        tracing::error!("{e:?}");
        std::process::exit(1);
    }
}

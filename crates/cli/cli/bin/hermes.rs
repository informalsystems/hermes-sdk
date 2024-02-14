#![recursion_limit = "256"]

use hermes_cli::application::HermesCli;
use hermes_cli::Result;
use hermes_cli_framework::application::boot;

fn main() -> Result<()> {
    boot::<HermesCli>()
}

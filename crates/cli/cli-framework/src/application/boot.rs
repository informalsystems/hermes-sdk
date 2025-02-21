use std::sync::Arc;

use eyre::eyre;
use hermes_runtime::types::runtime::HermesRuntime;

use crate::application::log::{enable_ansi, install_logger};
use crate::application::Application;
use crate::{output, Result};

pub fn boot<A>() -> Result<()>
where
    A: Application,
{
    let _ = stable_eyre::install();

    let app = A::parse_from_env();

    let with_color = enable_ansi();
    let with_json = app.json_output();
    install_logger(with_color, with_json);

    output::set_json(with_json);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| eyre!("failed to initialized tokio runtime: {e}"))?;

    let rt = HermesRuntime::new(Arc::new(rt));

    rt.runtime.block_on(run(rt.clone(), app))?;

    Ok(())
}

pub async fn run<A>(rt: HermesRuntime, app: A) -> Result<()>
where
    A: Application,
{
    let output = app.run(rt).await?;
    output.print();
    Ok(())
}

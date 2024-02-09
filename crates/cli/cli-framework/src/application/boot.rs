use std::sync::Arc;

use oneline_eyre::eyre::eyre;

use hermes_relayer_runtime::types::runtime::HermesRuntime;

use crate::application::log::{enable_ansi, install_logger};
use crate::application::Application;
use crate::config::Config;
use crate::output;
use crate::Result;

pub fn boot<A>() -> Result<()>
where
    A: Application,
{
    oneline_eyre::install()?;

    let app = A::parse_from_env();
    let config_path = app.config_path();

    let with_color = enable_ansi();
    let with_json = app.json_output();
    install_logger(with_color, with_json);

    output::set_json(with_json);

    let config =
        A::Config::load_from_path(config_path).map_err(|e| eyre!("failed to load config: {e}"))?;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| eyre!("failed to initialized tokio runtime: {e}"))?;

    let rt = HermesRuntime::new(Arc::new(rt));

    rt.runtime
        .block_on(run(rt.clone(), app, config))
        .map_err(|e| eyre!("Hermes command exited with an error: {e}"))?;

    Ok(())
}

pub async fn run<A>(rt: HermesRuntime, app: A, config: A::Config) -> Result<()>
where
    A: Application,
{
    let output = app.run(rt, config).await?;
    output.print();
    Ok(())
}

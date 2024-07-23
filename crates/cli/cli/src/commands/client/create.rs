use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::DestinationTarget;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::foreign_client::CreateOptions;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use oneline_eyre::eyre::eyre;
use tracing::info;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct ClientCreate {
    /// Identifier of the chain that hosts the client
    #[clap(
        long = "host-chain",
        required = true,
        value_name = "HOST_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    host_chain_id: ChainId,

    /// Identifier of the chain targeted by the client
    #[clap(
        long = "reference-chain",
        required = true,
        value_name = "REFERENCE_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    reference_chain_id: ChainId,

    /// The maximum allowed clock drift for this client.
    ///
    /// The clock drift is a correction parameter. It helps deal with clocks
    /// that are only approximately synchronized between the source and destination chains
    /// of this client.
    /// The destination chain for this client uses the clock drift parameter when deciding
    /// to accept or reject a new header (originating from the source chain) for this client.
    /// If this option is not specified, a suitable clock drift value is derived from the chain
    /// configurations.
    #[clap(long = "clock-drift", value_name = "CLOCK_DRIFT")]
    clock_drift: Option<humantime::Duration>,

    /// Override the trusting period specified in the config.
    ///
    /// The trusting period specifies how long a validator set is trusted for
    /// (must be shorter than the chain's unbonding period).
    #[clap(long = "trusting-period", value_name = "TRUSTING_PERIOD")]
    trusting_period: Option<humantime::Duration>,

    /// Override the trust threshold specified in the configuration.
    ///
    /// The trust threshold defines what fraction of the total voting power of a known
    /// and trusted validator set is sufficient for a commit to be accepted going forward.
    #[clap(
        long = "trust-threshold",
        value_name = "TRUST_THRESHOLD",
        value_parser = parse_trust_threshold
    )]
    trust_threshold: Option<TrustThreshold>,
}

impl CommandRunner<HermesApp> for ClientCreate {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let host_chain = builder.build_chain(&self.host_chain_id).await?;
        let reference_chain = builder.build_chain(&self.reference_chain_id).await?;

        let options = CreateOptions {
            max_clock_drift: self.clock_drift.map(|d| d.into()),
            trusting_period: self.trusting_period.map(|d| d.into()),
            trust_threshold: self.trust_threshold,
        };

        let settings = Settings::for_create_command(
            options,
            &host_chain.chain_config.clone(),
            &reference_chain.chain_config.clone(),
        );

        info!(
            ?settings,
            "Creating client on host chain `{}` that references chain `{}`...",
            self.host_chain_id,
            self.reference_chain_id
        );

        let client_id_on_host = CosmosRelay::create_client(
            DestinationTarget,
            &host_chain,
            &reference_chain,
            &settings,
            &(),
        )
        .await
        .map_err(|e| eyre!("Failed to create client on host chain: {e}"))?;

        info!(
            %client_id_on_host,
            "Successfully created client on host chain `{}`",
            self.host_chain_id,
        );

        Ok(Output::success_msg("Done"))
    }
}

fn parse_trust_threshold(input: &str) -> eyre::Result<TrustThreshold> {
    let (num_part, denom_part) = input
        .split_once('/')
        .ok_or_else(|| eyre!("expected a fractional argument, eg. '2/3'"))?;

    let numerator = num_part
        .trim()
        .parse()
        .map_err(|_| eyre!("invalid trust threshold numerator"))?;

    let denominator = denom_part
        .trim()
        .parse()
        .map_err(|_| eyre!("invalid trust threshold denominator"))?;

    let trust_threshold = TrustThreshold::new(numerator, denominator)
        .map_err(|e| eyre!("invalid trust threshold: {e}"))?;

    Ok(trust_threshold)
}

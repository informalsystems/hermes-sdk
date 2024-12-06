use cgp::prelude::*;
use eyre::eyre;
use ibc::clients::tendermint::types::TrustThreshold;

#[derive(Debug, clap::Parser, HasField)]
pub struct CreateClientArgs {
    /// Identifier of the chain that hosts the client
    #[clap(
        long = "target-chain",
        required = true,
        value_name = "TARGET_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    pub target_chain_id: String,

    /// Identifier of the chain targeted by the client
    #[clap(
        long = "counterparty-chain",
        required = true,
        value_name = "COUNTERPARTY_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    pub counterparty_chain_id: String,

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
    pub clock_drift: Option<humantime::Duration>,

    /// Override the trusting period specified in the config.
    ///
    /// The trusting period specifies how long a validator set is trusted for
    /// (must be shorter than the chain's unbonding period).
    #[clap(long = "trusting-period", value_name = "TRUSTING_PERIOD")]
    pub trusting_period: Option<humantime::Duration>,

    /// Override the trust threshold specified in the configuration.
    ///
    /// The trust threshold defines what fraction of the total voting power of a known
    /// and trusted validator set is sufficient for a commit to be accepted going forward.
    #[clap(
        long = "trust-threshold",
        value_name = "TRUST_THRESHOLD",
        value_parser = parse_trust_threshold
    )]
    pub trust_threshold: Option<TrustThreshold>,
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

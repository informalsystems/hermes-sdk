use cgp::prelude::*;
use core::time::Duration;

use ibc_client_tendermint::types::proto::v1::Fraction;

use crate::types::tendermint::{TendermintClientState, TendermintConsensusState, TendermintHeader};

#[derive(Debug, Clone)]
pub struct CosmosUpdateClientPayload {
    pub headers: Vec<TendermintHeader>,
}

#[derive(Debug, Clone, HasField)]
pub struct CosmosCreateClientPayload {
    pub client_state: TendermintClientState,
    pub consensus_state: TendermintConsensusState,
}

#[derive(Clone, Debug)]
pub struct CosmosCreateClientOptions {
    pub max_clock_drift: Duration,
    pub trusting_period: Duration,
    pub trust_threshold: Fraction,
}

impl Default for CosmosCreateClientOptions {
    fn default() -> Self {
        Self {
            max_clock_drift: Default::default(),
            // Set default to 14 days
            trusting_period: Duration::from_secs(14 * 24 * 60 * 60),
            trust_threshold: Fraction {
                numerator: 2,
                denominator: 3,
            },
        }
    }
}

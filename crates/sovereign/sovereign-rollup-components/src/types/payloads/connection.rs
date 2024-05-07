use core::time::Duration;
use ibc_relayer_types::core::ics03_connection::version::Version;

pub struct SovereignInitConnectionOptions {
    pub delay_period: Duration,
    pub connection_version: Version,
}

pub struct SovereignConnectionOpenInitRollupPayload {
    // TODO: fill in fields
}

pub struct SovereignConnectionOpenTryRollupPayload {
    // TODO: fill in fields
}

pub struct SovereignConnectionOpenAckRollupPayload {
    // TODO: fill in fields
}

pub struct SovereignConnectionOpenConfirmRollupPayload {
    // TODO: fill in fields
}

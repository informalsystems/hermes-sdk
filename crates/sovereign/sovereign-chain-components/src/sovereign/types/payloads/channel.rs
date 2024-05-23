use ibc_relayer_types::core::ics04_channel::version::Version;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc_relayer_types::Height;

pub struct SovereignInitChannelOptions {
    // TODO: fill in fields
}

pub struct SovereignChannelOpenTryPayload {
    // TODO: fill in fields
}

pub struct SovereignChannelOpenAckPayload {
    pub version: Version,
    pub update_height: Height,
    pub proof_try: CommitmentProofBytes,
}

pub struct SovereignChannelOpenConfirmPayload {
    // TODO: fill in fields
}

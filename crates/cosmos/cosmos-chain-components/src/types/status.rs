use ibc_relayer_types::Height;
pub use tendermint::Time;

pub struct ChainStatus {
    pub height: Height,
    pub time: Time,
}

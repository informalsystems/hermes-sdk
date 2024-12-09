use ibc::core::client::types::Height;
pub use tendermint::Time;

pub struct ChainStatus {
    pub height: Height,
    pub time: Time,
}

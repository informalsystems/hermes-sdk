use core::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub enum ClientStatus {
    Frozen,
    Expired,
    Active,
}

use core::fmt::{Debug, Display};

use crate::types::denom::MockDenom;

#[derive(Debug, Clone)]
pub struct MockAmount {
    pub quantity: u8,
    pub denom: MockDenom,
}

impl Display for MockAmount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

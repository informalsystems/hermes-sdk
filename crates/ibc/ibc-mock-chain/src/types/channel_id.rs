use core::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MockChannelId {
    ChannelIdA,
    ChannelIdB,
    ChannelIdC,
}

impl Display for MockChannelId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

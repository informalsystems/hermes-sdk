use core::fmt::{self, Display};
use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Denom {
    Base(String),
    Ibc {
        path: String,
        denom: String,
        hashed: String,
    },
}

impl Display for Denom {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Denom::Base(denom) => {
                write!(f, "{denom}")
            }
            Denom::Ibc { hashed, .. } => {
                write!(f, "{hashed}")
            }
        }
    }
}

impl FromStr for Denom {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(index) = s.find('/') {
            let (before, after) = s.split_at(index);
            Ok(Denom::Ibc {
                path: before.to_owned(),
                denom: "TBD".to_owned(),
                hashed: after[1..].to_owned(),
            })
        } else {
            Ok(Denom::Base(s.to_string()))
        }
    }
}

impl Denom {
    pub fn base(denom: &str) -> Self {
        Denom::Base(denom.to_string())
    }

    pub fn hash_only(&self) -> String {
        match self {
            Denom::Base(denom) => denom.to_string(),
            Denom::Ibc { hashed, .. } => match hashed.find('/') {
                Some(index) => hashed[index + 1..].to_string(),
                None => hashed.to_string(),
            },
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Denom::Base(denom) => denom,
            Denom::Ibc { hashed, .. } => hashed,
        }
    }
}

impl PartialEq for Denom {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Base(d1), Self::Base(d2)) => d1 == d2,
            (
                Self::Ibc {
                    path: p1,
                    denom: d1,
                    hashed: h1,
                },
                Self::Ibc {
                    path: p2,
                    denom: d2,
                    hashed: h2,
                },
            ) => p1 == p2 && d1 == d2 && h1 == h2,
            _ => self.as_str() == other.as_str(),
        }
    }
}

impl Eq for Denom {}

use core::fmt::Display;

use cgp_core::Async;

pub trait HasTokenType {
    type Token: Display + Async;
}

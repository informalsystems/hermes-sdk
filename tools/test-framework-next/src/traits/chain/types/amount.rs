use core::fmt::Display;

use cgp_core::Async;

pub trait HasAmountType: Async {
    type Amount: Display + Eq + PartialOrd + Async;
}

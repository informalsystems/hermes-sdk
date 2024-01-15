use core::fmt::Display;

use cgp_core::Async;

pub trait HasMemoType: Async {
    type Memo: Display + Async;
}

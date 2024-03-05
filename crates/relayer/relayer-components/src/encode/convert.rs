use cgp_core::prelude::*;

pub trait CanConvert<From, To>: HasErrorType {
    fn convert(&self, from: From) -> Result<To, Self::Error>;
}

use core::fmt::Display;

use hermes_cosmos_core::error::types::Error;

pub trait ErrorWrapper {
    type Value;

    fn wrap_error<M>(self, message: M) -> Result<Self::Value, Error>
    where
        M: Display;
}

impl<T, E> ErrorWrapper for Result<T, E>
where
    Error: From<E>,
{
    type Value = T;

    fn wrap_error<M>(self, message: M) -> Result<T, Error>
    where
        M: Display,
    {
        self.map_err(|e| Error::from(e).wrap(message))
    }
}

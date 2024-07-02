use cgp_core::error::{CanRaiseError, HasErrorType};

use crate::traits::convert::Converter;

pub struct TryConvertFrom;

impl<Encoding, From, To> Converter<Encoding, From, To> for TryConvertFrom
where
    Encoding: CanRaiseError<From::Error>,
    From: Clone + TryInto<To>,
{
    fn convert(_encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        from.clone().try_into().map_err(Encoding::raise_error)
    }
}

pub struct ConvertFrom;

impl<Encoding, From, To> Converter<Encoding, From, To> for ConvertFrom
where
    Encoding: HasErrorType,
    From: Clone + Into<To>,
{
    fn convert(_encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        Ok(from.clone().into())
    }
}

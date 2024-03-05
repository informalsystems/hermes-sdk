use cgp_core::CanRaiseError;

use crate::encode::traits::convert::Converter;

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

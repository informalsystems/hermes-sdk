use core::marker::PhantomData;

use hermes_prelude::*;

use crate::traits::{Converter, ConverterComponent};

pub struct TryConvertFrom;

#[cgp_provider(ConverterComponent)]
impl<Encoding, From, To> Converter<Encoding, From, To> for TryConvertFrom
where
    Encoding: CanRaiseAsyncError<From::Error>,
    From: Clone + TryInto<To>,
{
    fn convert(_encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        from.clone().try_into().map_err(Encoding::raise_error)
    }
}

pub struct ConvertFrom;

#[cgp_provider(ConverterComponent)]
impl<Encoding, From, To> Converter<Encoding, From, To> for ConvertFrom
where
    Encoding: HasAsyncErrorType,
    From: Clone + Into<To>,
{
    fn convert(_encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        Ok(from.clone().into())
    }
}

pub struct ConvertVia<ViaValue, ConvertA, ConvertB>(
    pub PhantomData<(ViaValue, ConvertA, ConvertB)>,
);

#[cgp_provider(ConverterComponent)]
impl<Encoding, From, To, ViaValue, ConvertA, ConvertB> Converter<Encoding, From, To>
    for ConvertVia<ViaValue, ConvertA, ConvertB>
where
    Encoding: HasAsyncErrorType,
    ConvertA: Converter<Encoding, From, ViaValue>,
    ConvertB: Converter<Encoding, ViaValue, To>,
{
    fn convert(encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        let intermediate = ConvertA::convert(encoding, from)?;
        ConvertB::convert(encoding, &intermediate)
    }
}

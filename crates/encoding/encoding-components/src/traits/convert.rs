use cgp_core::prelude::*;

#[derive_component(ConverterComponent, Converter<Encoding>)]
pub trait CanConvert<From, To>: HasErrorType {
    fn convert(&self, from: &From) -> Result<To, Self::Error>;
}

pub trait CanConvertBothWays<A, B>: CanConvert<A, B> + CanConvert<B, A> {}

impl<Encoding, A, B> CanConvertBothWays<A, B> for Encoding where
    Encoding: CanConvert<A, B> + CanConvert<B, A>
{
}

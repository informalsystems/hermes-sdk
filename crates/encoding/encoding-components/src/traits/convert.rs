use cgp_core::prelude::*;

#[derive_component(ConverterComponent, Converter<Encoding>)]
pub trait CanConvert<From, To>: HasErrorType {
    fn convert(&self, from: &From) -> Result<To, Self::Error>;
}

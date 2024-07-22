use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::traits::parse::ArgParser;

pub struct GetField;

impl<App, Args, Tag, Field> ArgParser<App, Args, Tag> for GetField
where
    App: HasErrorType,
    Args: HasField<Tag, Field = Field>,
    Field: Async + Clone,
{
    type Parsed = Field;

    fn parse_arg(_app: &App, args: &Args) -> Result<Self::Parsed, App::Error> {
        Ok(args.get_field(PhantomData).clone())
    }
}

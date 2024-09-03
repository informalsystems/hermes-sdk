use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::parse::ArgParser;

pub struct GetField;

impl<App, Args, Tag, Field> ArgParser<App, Args, Tag> for GetField
where
    App: HasErrorType,
    Args: HasField<Tag, Field = Field>,
    Field: Async + Clone,
{
    type Parsed = Field;

    fn parse_arg(
        _app: &App,
        args: &Args,
        _tag: PhantomData<Tag>,
    ) -> Result<Self::Parsed, App::Error> {
        Ok(args.get_field(PhantomData).clone())
    }
}

pub struct GetFieldWithTag<Tag>(pub PhantomData<Tag>);

impl<App, Args, TagA, TagB, Field> ArgParser<App, Args, TagA> for GetFieldWithTag<TagB>
where
    App: HasErrorType,
    Args: HasField<TagB, Field = Field>,
    Field: Async + Clone,
    TagB: Async,
{
    type Parsed = Field;

    fn parse_arg(
        _app: &App,
        args: &Args,
        _tag: PhantomData<TagA>,
    ) -> Result<Self::Parsed, App::Error> {
        Ok(args.get_field(PhantomData).clone())
    }
}

use core::marker::PhantomData;

use hermes_prelude::*;

use crate::traits::{ArgParser, ArgParserComponent};

pub struct GetField;

#[cgp_provider(ArgParserComponent)]
impl<App, Args, Tag, Field> ArgParser<App, Args, Tag> for GetField
where
    App: HasAsyncErrorType,
    Args: HasField<Tag, Value = Field>,
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

#[cgp_provider(ArgParserComponent)]
impl<App, Args, TagA, TagB, Field> ArgParser<App, Args, TagA> for GetFieldWithTag<TagB>
where
    App: HasAsyncErrorType,
    Args: HasField<TagB, Value = Field>,
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

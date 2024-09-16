use core::marker::PhantomData;

use cgp::prelude::HasField;

pub trait FieldGetter<Context, Tag> {
    type Field;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &Self::Field;
}

pub trait OwnedFieldGetter<Context> {
    type Field;

    fn get_field(context: &Context) -> Self::Field;
}

pub struct GetField;

impl<Context, Tag> FieldGetter<Context, Tag> for GetField
where
    Context: HasField<Tag>,
{
    type Field = Context::Field;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &Context::Field {
        context.get_field(tag)
    }
}

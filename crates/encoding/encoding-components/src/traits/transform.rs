pub trait Transformer {
    type From;
    type To;

    fn transform(from: Self::From) -> Self::To;
}

pub trait TransformerRef {
    type From;
    type To<'a>
    where
        Self: 'a;

    fn transform<'a>(from: &'a Self::From) -> Self::To<'a>;
}

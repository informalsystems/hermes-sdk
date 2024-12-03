use core::fmt::Display;

#[derive(Clone, Copy, Default)]
pub struct Index<const I: usize>;

#[derive(Clone, Copy)]
pub struct Twindex<const I: usize, const J: usize>;

impl<const I: usize> Display for Index<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        I.fmt(f)
    }
}

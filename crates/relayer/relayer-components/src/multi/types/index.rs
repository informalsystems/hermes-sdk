use core::fmt::Display;

#[derive(Clone, Copy, Default)]
pub struct Index<const I: usize>;

impl<const I: usize> Display for Index<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        I.fmt(f)
    }
}

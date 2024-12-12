use core::fmt::{Display, Error as FmtError, Formatter};

#[derive(Default)]
pub struct Memo(String);

impl Display for Memo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

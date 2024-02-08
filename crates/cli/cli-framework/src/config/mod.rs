use std::path::Path;

pub trait Config: Sized {
    fn load_from_path(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>>;
}

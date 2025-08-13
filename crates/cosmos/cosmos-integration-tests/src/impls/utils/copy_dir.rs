use std::path::PathBuf;

use hermes_error::HermesError;

pub fn copy_dir_recursive(
    source_dir: &PathBuf,
    destination_dir: &PathBuf,
) -> Result<(), HermesError> {
    if !destination_dir.exists() {
        std::fs::create_dir_all(destination_dir)?;
    }

    for entry in std::fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = destination_dir.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

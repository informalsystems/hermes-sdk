use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::PathBuf;

use ibc::core::host::types::identifiers::ChainId;

use super::secp256k1::{Secp256k1KeyPair, KEYSTORE_FILE_EXTENSION};
use crate::types::{KEYSTORE_DEFAULT_FOLDER, KEYSTORE_DISK_BACKEND};

#[derive(Clone, Debug)]
pub enum KeyRing {
    Test(Test),
}

#[derive(Clone, Debug)]
pub struct Test {
    account_prefix: String,
    store: PathBuf,
}

impl Test {
    pub fn new(account_prefix: String, store: PathBuf) -> Self {
        Self {
            account_prefix,
            store,
        }
    }

    pub fn add_key(&self, key_name: &str, key_pair: Secp256k1KeyPair) -> Result<(), String> {
        let mut filename = self.store.join(key_name);
        filename.set_extension(KEYSTORE_FILE_EXTENSION);

        let file = File::create(filename.clone())
            .map_err(|e| format!("failed to create file {}. Cause {e}", filename.display()))?;

        serde_json::to_writer_pretty(file, &key_pair)
            .map_err(|e| format!("failed to encore key to file. Cause: {e}"))?;

        Ok(())
    }

    pub fn remove_key(&mut self, key_name: &str) -> Result<(), String> {
        let mut filename = self.store.join(key_name);
        filename.set_extension(KEYSTORE_FILE_EXTENSION);

        fs::remove_file(filename.clone())
            .map_err(|e| format!("failed to remove file {}. Cause {e}", filename.display()))?;

        Ok(())
    }

    fn get_key(&self, key_name: &str) -> Result<Secp256k1KeyPair, String> {
        let mut key_file = self.store.join(key_name);
        key_file.set_extension(KEYSTORE_FILE_EXTENSION);

        if !key_file.as_path().exists() {
            return Err(format!("key file not found {}", key_file.display()));
        }

        let file = File::open(&key_file)
            .map_err(|e| format!("failed to open key file {}. Cause: {e}", key_file.display()))?;

        let key_entry = serde_json::from_reader(file).map_err(|e| {
            format!(
                "failed to decode key file {}. Cause: {e}",
                key_file.display()
            )
        })?;

        Ok(key_entry)
    }
}

impl KeyRing {
    pub fn new_secp256k1(
        account_prefix: &str,
        chain_id: &ChainId,
        ks_folder: &Option<PathBuf>,
    ) -> Self {
        let ks_folder = match ks_folder {
            Some(folder) => folder.to_owned(),
            None => {
                let home = dirs_next::home_dir().unwrap();
                home.join(KEYSTORE_DEFAULT_FOLDER)
            }
        };

        let folder = ks_folder
            .join(chain_id.as_str())
            .join(KEYSTORE_DISK_BACKEND);

        // Create keys folder if it does not exist
        fs::create_dir_all(&folder).unwrap();

        Self::Test(Test::new(account_prefix.to_string(), folder))
    }

    pub fn account_prefix(&self) -> &str {
        match self {
            Self::Test(keyring) => &keyring.account_prefix,
        }
    }

    pub fn add_key(&self, key_name: &str, key_pair: Secp256k1KeyPair) -> Result<(), String> {
        match self {
            Self::Test(keyring) => keyring.add_key(key_name, key_pair),
        }
    }

    pub fn remove_key(&mut self, key_name: &str) -> Result<(), String> {
        match self {
            Self::Test(keyring) => keyring.remove_key(key_name),
        }
    }

    pub fn get_key(&self, name: &str) -> Result<Secp256k1KeyPair, String> {
        match self {
            Self::Test(keyring) => keyring.get_key(name),
        }
    }

    pub fn keys(&self) -> Result<Vec<(String, Secp256k1KeyPair)>, String> {
        match self {
            Self::Test(keyring) => {
                let dir = fs::read_dir(&keyring.store).map_err(|e| {
                    format!(
                        "failed to read directory {}. Cause {e}",
                        keyring.store.as_path().display()
                    )
                })?;

                let ext = OsStr::new(KEYSTORE_FILE_EXTENSION);

                dir.into_iter()
                    .flatten()
                    .map(|entry| entry.path())
                    .filter(|path| path.extension() == Some(ext))
                    .flat_map(|path| path.file_stem().map(OsStr::to_owned))
                    .flat_map(|stem| stem.to_str().map(ToString::to_string))
                    .map(|name| self.get_key(&name).map(|key| (name, key)))
                    .collect()
            }
        }
    }
}

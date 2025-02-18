use std::{
    collections::HashMap,
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use anyhow::{bail, Context, Result};
use serde::Deserialize;

use crate::utils::path;

use super::Meta;

#[derive(Deserialize)]
pub struct Metas(HashMap<String, Meta>);
impl Metas {
    /// Get path
    fn get_path() -> PathBuf {
        path::get_data_dir().join("profile_metas.json")
    }

    /// Get global instance
    pub fn get_instance() -> &'static Mutex<Self> {
        static I: OnceLock<Mutex<Metas>> = OnceLock::new();
        I.get_or_init(|| {
            let path = Self::get_path();
            if path.is_file() {
                let contents = fs::read_to_string(&path)
                    .with_context(|| format!("try to read file `{}`", path.display()))
                    .unwrap();
                let value = serde_json::from_str::<HashMap<String, Meta>>(&contents)
                    .with_context(|| format!("try to parse file `{}`", path.display()))
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| (k.clone(), Meta { uuid: k, ..v }))
                    .collect::<HashMap<_, _>>();

                Mutex::new(Self(value))
            } else {
                Mutex::new(Self(HashMap::new()))
            }
        })
    }

    /// Flush to metadata file
    pub fn flush(&self) -> Result<()> {
        Ok(fs::write(
            Self::get_path(),
            serde_json::to_string(&self.0)?,
        )?)
    }

    /// Try to get metadata by uuid or name
    pub fn try_get_meta<S>(&self, uuid_or_name: S) -> Result<&Meta>
    where
        S: AsRef<str>,
    {
        if self.0.contains_key(uuid_or_name.as_ref()) {
            Ok(self.0.get(uuid_or_name.as_ref()).unwrap())
        } else {
            let candidates = self
                .0
                .iter()
                .filter(|(_, v)| v.name == uuid_or_name.as_ref())
                .map(|(_, v)| v)
                .collect::<Vec<_>>();
            if candidates.is_empty() {
                bail!("not found");
            }
            if candidates.len() > 1 {
                bail!("multiple found");
            }

            Ok(candidates.get(0).unwrap())
        }
    }
}
impl Deref for Metas {
    type Target = HashMap<String, Meta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Metas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

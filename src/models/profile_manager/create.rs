use eyre::{Context, Result};
use rand::{RngExt, rand_core::UnwrapErr, rngs::SysRng};
use sea_orm::{ActiveModelTrait, IntoActiveModel};

use crate::models::profile::profile;

use super::ProfileManager;

impl ProfileManager {
    pub async fn create(&self, mut model: profile::Model) -> Result<profile::Model> {
        // Set UUID
        model.uuid = Self::gen_uuid();

        // Insert database
        model
            .into_active_model()
            .insert(&**self)
            .await
            .wrap_err("failed to insert profile into database")
    }

    fn gen_uuid() -> String {
        const CHARSET: &str = "346789ABCDEFGHJKLMNPQRTUVWXYabcdefghijkmnpqrtwxyz";

        // Generate
        let mut rng = UnwrapErr(SysRng);
        let mut uuid = String::with_capacity(8);
        for _ in 0..8 {
            let idx = rng.random_range(0..CHARSET.len());
            uuid.push(CHARSET.as_bytes()[idx] as char);
        }

        // Done
        uuid
    }
}

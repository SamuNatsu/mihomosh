pub mod create;
pub mod fetch;

use std::{ops::Deref, sync::LazyLock};

use eyre::{Context, Result, eyre};
use sea_orm::{
    ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, IntoActiveModel,
};
use tokio::{fs, sync::OnceCell};
use url::Url;

use crate::models::profile::profile;

pub struct ProfileManager(DatabaseConnection);

impl Deref for ProfileManager {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ProfileManager {
    pub fn get_db_url() -> &'static Url {
        static INSTANCE: LazyLock<Url> = LazyLock::new(|| {
            let path = super::DATA_LOCAL_DIR.join("metadata.db");
            let path = path.to_string_lossy();
            let path = if let Some(str) = path.strip_prefix('/') {
                str
            } else {
                &path
            };

            // Create URL
            let mut url = Url::parse("sqlite:/:memory:").unwrap();
            url.set_path(path);
            url.set_query(Some("mode=rwc"));
            url
        });
        &INSTANCE
    }

    pub async fn get_instance() -> &'static Self {
        static INSTANCE: OnceCell<ProfileManager> = OnceCell::const_new();
        INSTANCE
            .get_or_init(|| async {
                // Create connection URL
                let url = Self::get_db_url();

                // Create database connection
                let db = Database::connect(url.as_str())
                    .await
                    .wrap_err_with(|| format!("failed to connect sqlite database `{}`", url))
                    .expect("sqlite database should be connectable");

                // Initialize schemas
                db.get_schema_builder()
                    .register(profile::Entity)
                    .sync(&db)
                    .await
                    .wrap_err("failed to build database schema")
                    .expect("database schema should be valid");

                // Done
                ProfileManager(db)
            })
            .await
    }

    pub async fn update_info(&self, model: profile::Model) -> Result<profile::Model> {
        // Create active model
        let is_local = model.r#type.is_local();
        let mut active_model = model.into_active_model();
        active_model.name.reset();
        active_model.r#type.reset();
        if is_local {
            active_model.url = ActiveValue::Set(None);
            active_model.user_agent = ActiveValue::Set(None);
            active_model.update_proxy = ActiveValue::Set(None);
            active_model.allow_invalid_certs = ActiveValue::Set(None);
        } else {
            active_model.url.reset();
            active_model.user_agent.reset();
            active_model.update_proxy.reset();
            active_model.allow_invalid_certs.reset();
        }

        // Update
        let uuid = active_model.uuid.try_as_ref().unwrap().clone();
        active_model
            .update(&**self)
            .await
            .wrap_err_with(|| format!("failed to update profile with UUID `{uuid}` into database"))
    }

    pub async fn delete<S: AsRef<str>>(&self, uuid: S) -> Result<()> {
        // Delete database entry
        let model = profile::Entity::delete_by_id(uuid.as_ref())
            .exec_with_returning(&**self)
            .await
            .wrap_err_with(|| {
                format!(
                    "failed to delete profile with UUID `{}` from database",
                    uuid.as_ref()
                )
            })?
            .ok_or_else(|| {
                eyre!(
                    "profile with UUID `{}` not exists in database",
                    uuid.as_ref()
                )
            })?;

        // Delete files
        fs::remove_file(model.get_data_path()).await.ok();
        fs::remove_file(model.get_ext_conf_path()).await.ok();
        fs::remove_file(model.get_ext_scr_path()).await.ok();

        // Done
        Ok(())
    }
}

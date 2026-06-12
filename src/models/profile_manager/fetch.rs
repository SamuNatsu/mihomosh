use eyre::{Context, Result, bail};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use strum::EnumIs;

use crate::{
    cli::profile::{ProfileGlobalTarget, ProfileTarget},
    models::profile::profile,
};

use super::ProfileManager;

#[derive(EnumIs)]
pub enum GlobalTargetFetchResult {
    Global,
    Profile(Box<profile::Model>),
}

impl ProfileManager {
    pub async fn fetch_all(&self) -> Result<Vec<profile::Model>> {
        profile::Entity::find()
            .all(&**self)
            .await
            .wrap_err("failed to fetch all profiles from database")
    }

    async fn fetch_by_uuid<S: AsRef<str>>(&self, uuid: S) -> Result<Option<profile::Model>> {
        profile::Entity::find_by_id(uuid.as_ref())
            .one(&**self)
            .await
            .wrap_err("failed to fetch profile by UUID from database")
    }

    async fn fetch_by_name<S: AsRef<str>>(&self, name: S) -> Result<Vec<profile::Model>> {
        profile::Entity::find()
            .filter(profile::Column::Name.eq(name.as_ref()))
            .limit(2)
            .all(&**self)
            .await
            .wrap_err("failed to fetch profile by name from database")
    }
}

impl ProfileTarget {
    pub async fn fetch_profile(&self) -> Result<Option<profile::Model>> {
        if let Some(uuid) = &self.uuid {
            ProfileManager::get_instance()
                .await
                .fetch_by_uuid(uuid)
                .await
        } else if let Some(name) = &self.name {
            let result = ProfileManager::get_instance()
                .await
                .fetch_by_name(name)
                .await;
            match result {
                Ok(profiles) => {
                    if profiles.len() > 1 {
                        bail!("multiple profiles found, please use UUID for precise search");
                    } else {
                        Ok(profiles.first().cloned())
                    }
                }
                Err(err) => Err(err),
            }
        } else {
            unreachable!()
        }
    }
}

impl ProfileGlobalTarget {
    pub async fn fetch_profile(&self) -> Result<Option<GlobalTargetFetchResult>> {
        if self.global {
            return Ok(Some(GlobalTargetFetchResult::Global));
        }

        Ok(ProfileTarget {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
        }
        .fetch_profile()
        .await?
        .map(|v| GlobalTargetFetchResult::Profile(Box::new(v))))
    }
}

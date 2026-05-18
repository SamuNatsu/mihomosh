use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel, Deserialize)]
#[serde(rename_all = "snake_case")]
#[sea_orm(table_name = "profile")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key, column_type = "String(StringLen::N(8))")]
    pub uuid: String,
    #[sea_orm(indexed)]
    pub name: String,
    pub r#type: ProfileType,
    pub url: Option<UrlString>,
    pub user_agent: Option<String>,
    pub update_proxy: Option<ProfileUpdateProxy>,
    pub allow_invalid_certs: Option<bool>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTimeUtc>,
    #[serde(skip_deserializing)]
    pub expired_at: Option<DateTimeUtc>,
    #[serde(skip_deserializing)]
    pub used_bytes: Option<i64>,
    #[serde(skip_deserializing)]
    pub total_bytes: Option<i64>,
}

#[derive(Clone, Debug, DeriveActiveEnum, Deserialize, EnumIter, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(8))",
    rename_all = "kebab-case"
)]
pub enum ProfileType {
    Remote,
    Local,
}

#[derive(Clone, Debug, DeriveActiveEnum, Deserialize, EnumIter, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(8))",
    rename_all = "kebab-case"
)]
pub enum ProfileUpdateProxy {
    None,
    System,
    Mihomo,
}

#[derive(Clone, Debug, Deserialize, Eq, FromJsonQueryResult, PartialEq, Serialize)]
pub struct UrlString(pub Url);

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use sea_orm::{Database, IntoActiveModel};

    use super::*;

    #[test]
    fn deser() {
        serde_json::from_str::<ModelEx>(
            r#"{
            "name": "Test",
            "type": "remote",
            "url": "http://127.0.0.1:9090",
            "user_agent": "mihomosh (clash-verge)",
            "update_proxy": "none",
            "allow_invalid_certs": false
        }"#,
        )
        .expect("fail to deserialize");
    }

    #[tokio::test]
    async fn db() {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("fail to create memory database");

        db.get_schema_builder()
            .register(Entity)
            .apply(&db)
            .await
            .expect("fail to register schema");

        let profile = serde_json::from_str::<ModelEx>(
            r#"{
                "uuid": "ignored",
                "name": "Test",
                "type": "remote",
                "url": "http://127.0.0.1:9090",
                "user_agent": "mihomosh (clash-verge)",
                "update_proxy": "none",
                "allow_invalid_certs": false,
                "updated_at": "ignored",
                "expired_at": "ignored",
                "used_bytes": "ignored",
                "total_bytes": "ignored"
            }"#,
        )
        .expect("fail to deserialize")
        .into_active_model()
        .set_uuid("abcdABCD")
        .insert(&db)
        .await
        .expect("fail to insert");
        println!("{profile:?}");

        let profile = Entity::find().one(&db).await.expect("fail to find");
        println!("{profile:?}")
    }
}

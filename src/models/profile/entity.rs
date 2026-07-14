use askama::Template;
use eyre::Result;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use smart_default::SmartDefault;
use strum::{AsRefStr, Display, EnumIs};
use url::Url;
use validator::{Validate, ValidationError};

pub const DEFAULT_USER_AGENT: &str =
    concat!("mihomosh/v", env!("CARGO_PKG_VERSION"), " (clash-verge)");

#[sea_orm::model]
#[derive(
    Clone, Debug, DeriveEntityModel, Deserialize, Serialize, SmartDefault, Template, Validate,
)]
#[template(path = "profile.template", escape = "yml")]
#[serde(rename_all = "kebab-case")]
#[validate(schema(function = "Self::validate"))]
#[sea_orm(table_name = "profile")]
pub struct Model {
    #[serde(skip)]
    #[sea_orm(primary_key, column_type = "String(StringLen::N(8))")]
    pub uuid: String,
    #[default = "New Profile"]
    #[serde(deserialize_with = "serde_trim::string_trim")]
    #[validate(length(min = 1, message = "CANNOT be empty"))]
    #[sea_orm(indexed)]
    pub name: String,
    pub r#type: ProfileType,
    pub url: Option<UrlString>,
    #[serde(default, deserialize_with = "serde_trim::option_string_trim")]
    pub user_agent: Option<String>,
    pub update_proxy: Option<ProfileUpdateProxy>,
    pub allow_invalid_certs: Option<bool>,
    #[serde(skip)]
    pub updated_at: Option<DateTimeUtc>,
    #[serde(skip)]
    pub expired_at: Option<DateTimeUtc>,
    #[serde(skip)]
    pub used_bytes: Option<i64>,
    #[serde(skip)]
    pub total_bytes: Option<i64>,
    #[serde(skip_deserializing)]
    #[sea_orm(ignore)]
    pub render_opts: Option<JsonValue>,
}

#[derive(
    AsRefStr,
    Clone,
    Copy,
    Debug,
    Default,
    DeriveActiveEnum,
    Deserialize,
    Display,
    EnumIs,
    EnumIter,
    Eq,
    PartialEq,
    Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(8))",
    rename_all = "kebab-case"
)]
pub enum ProfileType {
    #[default]
    Local,
    Remote,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    DeriveActiveEnum,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    PartialEq,
    Serialize,
)]
#[serde(rename_all = "kebab-case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(8))",
    rename_all = "kebab-case"
)]
pub enum ProfileUpdateProxy {
    #[default]
    None,
    System,
    Mihomo,
}

#[derive(Clone, Debug, Deserialize, Eq, FromJsonQueryResult, PartialEq, Serialize)]
pub struct UrlString(pub Url);

impl ActiveModelBehavior for ActiveModel {}

macro_rules! validate_function {
    () => {
        fn validate(&self) -> Result<(), ValidationError> {
            if self.r#type.is_remote() && self.url.is_none() {
                Err(ValidationError::new("schema")
                    .with_message("remote profile MUST has URL".into()))
            } else {
                Ok(())
            }
        }
    };
}

impl Model {
    validate_function!();
}

impl ModelEx {
    validate_function!();
}

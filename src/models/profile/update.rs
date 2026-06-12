use chrono::{DateTime, Utc};
use eyre::{Context, Result, bail};
use reqwest::{Client, Proxy, Response};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, IntoActiveModel};
use tokio::fs;

use crate::models::config::Config;

use super::entity::*;

impl Model {
    pub async fn update(&mut self, db: &DatabaseConnection) -> Result<()> {
        // If is local
        if self.r#type.is_local() {
            bail!("local profile is not allowed to be updated");
        }

        // Get response
        let url = self
            .url
            .clone()
            .expect("URL should not be empty for remote profile")
            .0;
        let resp = self
            .create_client()?
            .get(url.clone())
            .send()
            .await
            .wrap_err_with(|| format!("failed to send request to `{}`", url))?
            .error_for_status()
            .wrap_err_with(|| format!("failed to receive response from `{}`", url))?;

        // Update metadata
        *self = self
            .parse_header(&resp)
            .update(db)
            .await
            .wrap_err_with(|| {
                format!(
                    "failed to update profile with UUID `{}` into database",
                    self.uuid
                )
            })?;

        // Save contents
        let path = self.get_data_path();
        let contents = resp
            .text()
            .await
            .wrap_err_with(|| format!("failed to download profile from `{}`", url))?;
        fs::write(&path, &contents)
            .await
            .wrap_err_with(|| format!("failed to write file `{}`", path.display()))?;

        // Done
        Ok(())
    }

    pub async fn update_data(&mut self, db: &DatabaseConnection) -> Result<()> {
        if self.r#type.is_local() {
            // Create active model
            let mut model = self.clone().into_active_model();
            model.updated_at = ActiveValue::Set(Some(Utc::now()));

            // Update metadata
            *self = model.update(db).await.wrap_err_with(|| {
                format!(
                    "failed to update profile with UUID `{}` into database",
                    self.uuid
                )
            })?;
        }

        // Done
        return Ok(());
    }

    fn create_client(&self) -> Result<Client> {
        // Create builder
        let mut builder = Client::builder()
            .user_agent(self.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT))
            .danger_accept_invalid_certs(self.allow_invalid_certs.unwrap_or(false));

        // Set proxy
        if let Some(proxy) = &self.update_proxy {
            builder = match proxy {
                ProfileUpdateProxy::None => builder.no_proxy(),
                ProfileUpdateProxy::System => builder,
                ProfileUpdateProxy::Mihomo => {
                    let proxy = Proxy::all(format!(
                        "http://127.0.0.1:{}",
                        Config::get_instance().lock().unwrap().port
                    ))
                    .unwrap();
                    builder.proxy(proxy)
                }
            }
        }

        // Build client
        builder.build().wrap_err("failed to create HTTP client")
    }

    fn parse_header(&self, resp: &Response) -> ActiveModel {
        resp.headers()
            .get("Subscription-Userinfo")
            .and_then(|header| {
                // Create active model
                let mut model = self.clone().into_active_model();
                model.updated_at = ActiveValue::Set(Some(Utc::now()));
                model.expired_at = ActiveValue::Set(None);
                model.used_bytes = ActiveValue::Set(None);
                model.total_bytes = ActiveValue::Set(None);

                // Parse key value pairs
                let kvp = header
                    .to_str()
                    .ok()?
                    .split(';')
                    .filter_map(|kv| {
                        kv.trim()
                            .split_once('=')
                            .and_then(|(k, v)| match v.parse() {
                                Ok(v) => Some((k, v)),
                                Err(_) => None,
                            })
                    })
                    .collect::<Vec<_>>();

                // Update active model
                for (k, v) in kvp {
                    match k {
                        "upload" | "download" => {
                            model.used_bytes = if let ActiveValue::Set(Some(tmp)) = model.used_bytes
                            {
                                ActiveValue::Set(Some(tmp + v))
                            } else {
                                ActiveValue::Set(Some(v))
                            }
                        }
                        "total" => model.total_bytes = ActiveValue::Set(Some(v)),
                        "expire" => {
                            model.expired_at = ActiveValue::Set(DateTime::from_timestamp_secs(v))
                        }
                        _ => (),
                    }
                }

                // Done
                Some(model)
            })
            .unwrap_or_else(|| {
                let mut model = self.clone().into_active_model();
                model.updated_at = ActiveValue::Set(Some(Utc::now()));
                model
            })
    }
}

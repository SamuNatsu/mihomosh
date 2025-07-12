pub mod resp;

use std::{collections::HashMap, sync::Arc};

use anyhow::{Context, Error, Result, anyhow};
use bytes::Bytes;
use futures::{Stream, StreamExt, stream};
use reqwest::{ClientBuilder, Method, RequestBuilder, Response};
use serde::Deserialize;
use serde_json::{Value, json};
use url::Url;

use crate::models::config::Config;

pub struct Api {
    api: Url,
    secret: Option<String>,
}

impl Api {
    fn wrap_chunk_stream<T, U>(resp: Response, parser: T) -> impl Stream<Item = Result<U>>
    where
        T: Fn(Bytes) -> Result<U> + Send + Sync + Clone + 'static,
    {
        let parser = Arc::new(parser);
        stream::unfold((resp, parser), |(mut resp, parser)| async move {
            match resp.chunk().await {
                Ok(Some(bytes)) => Some((parser(bytes), (resp, parser))),
                Ok(None) => None,
                Err(e) => Some((Err(Error::from(e)), (resp, parser))),
            }
        })
        .boxed()
    }

    fn create_request_builder<S: AsRef<str>>(
        &self,
        method: Method,
        path: S,
    ) -> Result<RequestBuilder> {
        let mut url = self.api.clone();
        url.set_path(path.as_ref());

        let mut builder = ClientBuilder::new()
            .no_proxy()
            .user_agent(format!(
                "mihomosh/v{} (clash-verge)",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .context("Fail to build reqwest client")?
            .request(method, url);

        if let Some(secret) = &self.secret {
            builder = builder.bearer_auth(secret);
        }

        Ok(builder)
    }

    pub async fn get_logs(&self) -> Result<impl Stream<Item = Result<(String, String)>>> {
        #[derive(Deserialize)]
        struct RespBody {
            r#type: String,
            payload: String,
        }

        let resp = self
            .create_request_builder(Method::GET, "logs")?
            .send()
            .await
            .context("Fail to send `GET /logs`")?
            .error_for_status()
            .context("Fail to request `GET /logs`")?;

        let stream = Self::wrap_chunk_stream(resp, |bytes| {
            serde_json::from_slice::<RespBody>(&bytes)
                .map(|body| (body.r#type, body.payload))
                .map_err(anyhow::Error::from)
        });

        Ok(stream)
    }

    pub async fn get_traffic(&self) -> Result<impl Stream<Item = Result<(u64, u64)>>> {
        #[derive(Deserialize)]
        struct RespBody {
            up: u64,
            down: u64,
        }

        let resp = self
            .create_request_builder(Method::GET, "traffic")?
            .send()
            .await
            .context("Fail to send `GET /traffic`")?
            .error_for_status()
            .context("Fail to request `GET /traffic`")?;

        let stream = Self::wrap_chunk_stream(resp, |bytes| {
            serde_json::from_slice::<RespBody>(&bytes)
                .map(|body| (body.up, body.down))
                .map_err(anyhow::Error::from)
        });

        Ok(stream)
    }

    pub async fn get_memory(&self) -> Result<impl Stream<Item = Result<u64>>> {
        #[derive(Deserialize)]
        struct RespBody {
            inuse: u64,
        }

        let resp = self
            .create_request_builder(Method::GET, "memory")?
            .send()
            .await
            .context("Fail to send `GET /memory`")?
            .error_for_status()
            .context("Fail to request `GET /memory`")?;

        let stream = Self::wrap_chunk_stream(resp, |bytes| {
            serde_json::from_slice::<RespBody>(&bytes)
                .map(|body| body.inuse)
                .map_err(anyhow::Error::from)
        });

        Ok(stream)
    }

    pub async fn get_version(&self) -> Result<String> {
        #[derive(Deserialize)]
        struct RespBody {
            version: String,
        }

        let body = self
            .create_request_builder(Method::GET, "version")?
            .send()
            .await
            .context("Fail to send `GET /version`")?
            .error_for_status()
            .context("Fail to request `GET /version`")?
            .json::<RespBody>()
            .await
            .context("Fail to parse response from `GET /memory`")?;

        Ok(body.version)
    }

    pub async fn flush_fake_ip_cache(&self) -> Result<()> {
        self.create_request_builder(Method::POST, "cache/fakeip/flush")?
            .send()
            .await
            .context("Fail to send `POST /cache/fakeip/flush`")?
            .error_for_status()
            .context("Fail to request `POST /cache/fakeip/flush`")?;
        Ok(())
    }

    pub async fn restart(&self) -> Result<()> {
        self.create_request_builder(Method::POST, "restart")?
            .send()
            .await
            .context("Fail to send `POST /restart`")?
            .error_for_status()
            .context("Fail to request `POST /restart`")?;
        Ok(())
    }

    pub async fn upgrade_ui(&self) -> Result<()> {
        self.create_request_builder(Method::POST, "upgrade/ui")?
            .send()
            .await
            .context("Fail to send `POST /upgrade/ui`")?
            .error_for_status()
            .context("Fail to request `POST /upgrade/ui`")?;
        Ok(())
    }

    pub async fn upgrade_geo(&self) -> Result<()> {
        self.create_request_builder(Method::POST, "upgrade/geo")?
            .send()
            .await
            .context("Fail to send `POST /upgrade/geo`")?
            .error_for_status()
            .context("Fail to request `POST /upgrade/geo`")?;
        Ok(())
    }

    /// NEEDS REFACTOR
    pub async fn get_proxies(&self) -> Result<Vec<resp::Proxy>> {
        let url = format!("{}/version", self.api);
        let ret = self
            .create_request_builder(Method::GET, url)?
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?
            .as_object()
            .ok_or(anyhow!("invalid response body"))?
            .get("proxies")
            .ok_or(anyhow!("invalid response body"))?
            .clone();
        let ret = serde_json::from_value::<Vec<resp::Proxy>>(ret)?;

        Ok(ret)
    }

    /// NEEDS REFACTOR
    pub async fn select_proxy<S1, S2>(&self, proxy: S1, name: S2) -> Result<()>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let url = format!(
            "{}/proxies/{}",
            self.api,
            urlencoding::encode(proxy.as_ref())
        );
        let body = serde_json::to_string(&json!({ "name": name.as_ref() }))?;
        self.create_request_builder(Method::PUT, url)?
            .body(body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// NEEDS REFACTOR
    pub async fn test_proxy<S1, S2>(&self, proxy: S1, url: S2, delay: u64) -> Result<i64>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let url = format!(
            "{}/proxies/{}?url={}&timeout={}",
            self.api,
            urlencoding::encode(proxy.as_ref()),
            urlencoding::encode(url.as_ref()),
            delay
        );
        let ret = self
            .create_request_builder(Method::GET, url)?
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?
            .as_object()
            .ok_or(anyhow!("invalid response body"))?
            .get("delay")
            .ok_or(anyhow!("invalid response body"))?
            .as_i64()
            .ok_or(anyhow!("invalid response body"))?;

        Ok(ret)
    }

    pub async fn get_rules(&self) -> Result<Vec<resp::Rule>> {
        let ret = self
            .create_request_builder(Method::GET, "rules")?
            .send()
            .await
            .context("Fail to send `GET /rules`")?
            .error_for_status()
            .context("Fail to request `GET /rules`")?
            .json::<Value>()
            .await
            .context("Fail to parse response from `GET /rules`")?
            .as_object()
            .ok_or(anyhow!("Not an object"))
            .context("Fail to parse response from `GET /rules`")?
            .get("rules")
            .ok_or(anyhow!("`rules` key not found"))
            .context("Fail to parse response from `GET /rules`")?
            .clone();
        let ret = serde_json::from_value::<Option<Vec<resp::Rule>>>(ret)
            .context("Fail to parse response from `GET /rules`")?
            .unwrap_or_default();

        Ok(ret)
    }

    pub async fn get_rule_sets(&self) -> Result<HashMap<String, resp::RuleSet>> {
        let ret = self
            .create_request_builder(Method::GET, "providers/rules")?
            .send()
            .await
            .context("Fail to send `GET /providers/rules`")?
            .error_for_status()
            .context("Fail to request `GET /providers/rules`")?
            .json::<Value>()
            .await
            .context("Fail to parse response from `GET /providers/rules`")?
            .as_object()
            .ok_or(anyhow!("Not an object"))
            .context("Fail to parse response from `GET /providers/rules`")?
            .get("providers")
            .ok_or(anyhow!("`providers` key not found"))
            .context("Fail to parse response from `GET /providers/rules`")?
            .clone();
        let ret = serde_json::from_value::<Option<HashMap<String, resp::RuleSet>>>(ret)
            .context("Fail to parse response from `GET /providers/rules`")?
            .unwrap_or_default();

        Ok(ret)
    }

    pub async fn update_rule_set<S: AsRef<str>>(&self, name: S) -> Result<()> {
        let path = format!("providers/rules/{}", urlencoding::encode(name.as_ref()));
        self.create_request_builder(Method::PUT, &path)?
            .send()
            .await
            .with_context(|| format!("Fail to send `PUT /{path}`"))?
            .error_for_status()
            .with_context(|| format!("Fail to send `PUT /{path}`"))?;
        Ok(())
    }

    pub async fn get_connections(&self) -> Result<Vec<resp::Connection>> {
        let ret = self
            .create_request_builder(Method::GET, "connections")?
            .send()
            .await
            .context("Fail to send `GET /connections`")?
            .error_for_status()
            .context("Fail to request `GET /connections`")?
            .json::<Value>()
            .await
            .context("Fail to parse response from `GET /connections`")?
            .as_object()
            .ok_or(anyhow!("Not an object"))
            .context("Fail to parse response from `GET /connections`")?
            .get("connections")
            .ok_or(anyhow!("`connections` key not found"))
            .context("Fail to parse response from `GET /connections`")?
            .clone();
        let ret = serde_json::from_value::<Option<Vec<resp::Connection>>>(ret)
            .context("Fail to parse response from `GET /connections`")?
            .unwrap_or_default();

        Ok(ret)
    }

    pub async fn close_all_connections(&self) -> Result<()> {
        self.create_request_builder(Method::DELETE, "connections")?
            .send()
            .await
            .context("Fail to send `DELETE /connections`")?
            .error_for_status()
            .context("Fail to request `DELETE /connections`")?;
        Ok(())
    }

    pub async fn close_connection<S: AsRef<str>>(&self, id: S) -> Result<()> {
        let path = format!("connections/{}", urlencoding::encode(id.as_ref()));
        self.create_request_builder(Method::DELETE, &path)?
            .send()
            .await
            .with_context(|| format!("Fail to send `DELETE /{path}`"))?
            .error_for_status()
            .with_context(|| format!("Fail to send `DELETE /{path}`"))?;
        Ok(())
    }
}

impl Config {
    pub fn get_api(&self) -> Api {
        Api {
            api: self.mihomo_api.clone(),
            secret: self.mihomo_secret.clone(),
        }
    }
}

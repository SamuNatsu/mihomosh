use std::sync::Arc;

use anyhow::{Error, Result, anyhow, bail};
use bytes::Bytes;
use futures::{Stream, StreamExt, stream};
use reqwest::{ClientBuilder, IntoUrl, Method, RequestBuilder, Response};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::models::config::Config;

pub struct Api {
    api: String,
    secret: Option<String>,
}

impl Api {
    pub fn new<S1, S2>(api: S1, secret: Option<S2>) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            api: api.as_ref().to_owned(),
            secret: secret.map(|s| s.as_ref().to_owned()),
        }
    }

    fn create_request_builder<U: IntoUrl>(&self, method: Method, url: U) -> Result<RequestBuilder> {
        let mut builder = ClientBuilder::new()
            .no_proxy()
            .user_agent(format!(
                "mihomosh/v{} (clash-verge)",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?
            .request(method, url);

        if let Some(secret) = &self.secret {
            builder = builder.bearer_auth(secret);
        }

        Ok(builder)
    }

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

    pub async fn get_logs(&self) -> Result<impl Stream<Item = Result<(String, String)>>> {
        #[derive(Deserialize)]
        struct RespBody {
            r#type: String,
            payload: String,
        }

        let url = format!("{}/logs", self.api);
        let resp = self
            .create_request_builder(Method::GET, &url)?
            .send()
            .await?
            .error_for_status()?;
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

        let url = format!("{}/traffic", self.api);
        let resp = self
            .create_request_builder(Method::GET, &url)?
            .send()
            .await?
            .error_for_status()?;
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

        let url = format!("{}/memory", self.api);
        let resp = self
            .create_request_builder(Method::GET, &url)?
            .send()
            .await?
            .error_for_status()?;
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
            meta: Option<bool>,
            version: String,
        }

        let url = format!("{}/version", self.api);
        let body = self
            .create_request_builder(Method::GET, &url)?
            .send()
            .await?
            .error_for_status()?
            .json::<RespBody>()
            .await?;

        if !body.meta.unwrap_or_default() {
            bail!("not a Mihomo kernal");
        }

        Ok(body.version)
    }

    pub async fn flush_fake_ip_cache(&self) -> Result<()> {
        let url = format!("{}/cache/fakeip/flush", self.api);
        self.create_request_builder(Method::POST, url)?
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn restart(&self) -> Result<()> {
        let url = format!("{}/restart", self.api);
        self.create_request_builder(Method::POST, url)?
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn upgrade_ui(&self) -> Result<()> {
        let url = format!("{}/upgrade/ui", self.api);
        self.create_request_builder(Method::POST, url)?
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn upgrade_geo(&self) -> Result<()> {
        let url = format!("{}/upgrade/geo", self.api);
        self.create_request_builder(Method::POST, url)?
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

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

    pub async fn get_connections(&self) -> Result<Vec<resp::Connection>> {
        let url = format!("{}/connections", self.api);
        let ret = self
            .create_request_builder(Method::GET, url)?
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?
            .as_object()
            .ok_or(anyhow!("invalid response body"))?
            .get("connections")
            .ok_or(anyhow!("invalid response body"))?
            .clone();
        let ret = serde_json::from_value::<Option<Vec<resp::Connection>>>(ret)?;

        Ok(ret.unwrap_or_default())
    }

    pub async fn close_all_connections(&self) -> Result<()> {
        let url = format!("{}/connections", self.api);
        self.create_request_builder(Method::DELETE, url)?
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn close_connection<S: AsRef<str>>(&self, id: S) -> Result<()> {
        let url = format!(
            "{}/connections/{}",
            self.api,
            urlencoding::encode(id.as_ref())
        );
        self.create_request_builder(Method::DELETE, url)?
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

impl Config {
    pub fn get_api(&self) -> Api {
        Api::new(&self.mihomo_api, self.mihomo_secret.as_deref())
    }
}

pub mod resp {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Proxy {
        pub alive: bool,
        pub history: ProxyHistory,
        pub id: String,
        pub all: Vec<String>,
        pub name: String,
        pub now: String,
        pub r#type: String,
        pub udp: bool,
    }

    #[derive(Deserialize)]
    pub struct ProxyHistory {
        pub time: String,
        pub delay: i64,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Connection {
        pub id: String,
        pub metadata: ConnectionMetadata,
        pub chains: Vec<String>,
        pub start: String,
        pub rule: String,
        pub rule_payload: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectionMetadata {
        pub network: String,
        pub r#type: String,
        pub host: String,
        pub process: String,
        #[serde(rename(deserialize = "sourceIP"))]
        pub source_ip: String,
        pub source_port: String,
        #[serde(rename(deserialize = "destinationIP"))]
        pub destination_ip: String,
        pub destination_port: String,
    }
}

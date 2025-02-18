use anyhow::Result;
use reqwest::{Client, Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

pub struct Api {
    entry_point: String,
    auth_token: Option<String>,
}
impl Api {
    pub fn new<S1, S2>(entry_point: S1, auth_token: Option<S2>) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            entry_point: entry_point.as_ref().into(),
            auth_token: auth_token.map(|v| v.as_ref().into()),
        }
    }

    fn create_client<S: AsRef<str>>(&self, method: Method, path: S) -> Result<RequestBuilder> {
        let url = Url::parse(&self.entry_point)?.join(path.as_ref())?;
        if let Some(token) = &self.auth_token {
            Ok(Client::new().request(method, url).bearer_auth(token))
        } else {
            Ok(Client::new().request(method, url))
        }
    }

    pub async fn get_version(&self) -> Result<String> {
        // Get response
        #[derive(Deserialize)]
        struct Resp {
            version: String,
        }
        let body = self
            .create_client(Method::GET, "/version")?
            .send()
            .await?
            .text()
            .await?;
        let r = serde_json::from_str::<Resp>(&body)?;

        // Success
        Ok(r.version)
    }

    pub async fn get_configs(&self) -> Result<Value> {
        // Get response
        let body = self
            .create_client(Method::GET, "/configs")?
            .send()
            .await?
            .text()
            .await?;

        // Success
        Ok(serde_json::from_str(&body)?)
    }

    pub async fn restart(&self) -> Result<()> {
        // Get response
        self.create_client(Method::POST, "/restart")?
            .send()
            .await?
            .error_for_status()?;

        // Success
        Ok(())
    }

    pub async fn upgrade_geo(&self) -> Result<()> {
        // Get response
        self.create_client(Method::POST, "/upgrade/geo")?
            .send()
            .await?
            .error_for_status()?;

        // Success
        Ok(())
    }

    pub async fn get_groups(&self) -> Result<Value> {
        // Get response
        let body = self
            .create_client(Method::GET, "/group")?
            .send()
            .await?
            .text()
            .await?;

        // Success
        Ok(serde_json::from_str(&body)?)
    }

    pub async fn update_proxy<S1, S2>(&self, proxy: S1, selection: S2) -> Result<()>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        #[derive(Serialize)]
        struct Body {
            name: String,
        }

        // Get response
        let proxy = urlencoding::encode(proxy.as_ref());
        self.create_client(Method::PUT, format!("/proxies/{proxy}"))?
            .body(serde_json::to_string(&Body {
                name: selection.as_ref().to_owned(),
            })?)
            .send()
            .await?
            .error_for_status()?;

        // Success
        Ok(())
    }

    pub async fn get_connections(&self) -> Result<Value> {
        // Get response
        let body = self
            .create_client(Method::GET, "/connections")?
            .send()
            .await?
            .text()
            .await?;

        // Success
        Ok(serde_json::from_str(&body)?)
    }
}

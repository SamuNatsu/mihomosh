use anyhow::{Context, Result};
use chrono::{DateTime, Local};
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

#[derive(Deserialize)]
pub struct Rule {
    pub r#type: String,
    pub payload: String,
    pub proxy: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSet {
    pub name: String,
    pub vehicle_type: String,
    pub r#type: String,
    pub behavior: String,
    pub update_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub id: String,
    pub metadata: ConnectionMetadata,
    pub chains: Vec<String>,
    pub start: String,
    pub rule: String,
    pub rule_payload: String,
}

#[derive(Deserialize)]
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

impl Connection {
    pub fn get_start(&self) -> Result<String> {
        let ret = self
            .start
            .parse::<DateTime<Local>>()
            .with_context(|| format!("Fail to parse time `{}`", self.start))?
            .format("%Y-%m-%dT%H:%M:%S.%3f%:z")
            .to_string();
        Ok(ret)
    }

    pub fn get_rule(&self) -> String {
        format!(
            "{}{}",
            self.rule,
            if self.rule_payload.is_empty() {
                "".to_owned()
            } else {
                format!("({})", self.rule_payload)
            }
        )
    }

    pub fn get_type(&self) -> String {
        format!("{}({})", self.metadata.r#type, self.metadata.network)
    }

    pub fn get_src(&self) -> String {
        format!("{}:{}", self.metadata.source_ip, self.metadata.source_port)
    }

    pub fn get_dst(&self) -> String {
        format!(
            "{}:{}",
            self.metadata.destination_ip, self.metadata.destination_port
        )
    }
}

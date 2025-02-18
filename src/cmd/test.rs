use std::{
    error::Error,
    time::{Duration, Instant},
};

use anyhow::Result;
use reqwest::{Client, Proxy};

use crate::{
    data::config::Config,
    utils::result::{fail, success},
};

pub async fn test(url: String) -> Result<()> {
    let cfg = Config::get_instance();

    // Testing
    println!("{}", console::style(format!("Testing `{url}`")).green());
    let st = Instant::now();
    let r = Client::builder()
        .proxy(Proxy::all(format!("http://localhost:{}", cfg.mixed_port))?)
        .timeout(Duration::from_secs(10))
        .build()?
        .get(url)
        .send()
        .await;
    let ed = Instant::now();

    // Print result
    match r {
        Ok(_) => {
            let dur = ed - st;
            success!("Test passed in {}ms", dur.as_millis())
        }
        Err(err) => {
            let dur = ed - st;

            match err.source() {
                Some(src) => fail!(
                    "Test failed in {}ms: {err}\nCaused by: {}",
                    dur.as_millis(),
                    src
                ),
                None => fail!("Test failed in {}ms: {err}", dur.as_millis()),
            }
        }
    }
}

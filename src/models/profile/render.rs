use std::{any::Any, collections::HashMap};

use askama::Template;
use eyre::Result;

use super::entity::*;

impl Model {
    pub fn render(&self) -> Result<String> {
        let mut values: HashMap<&str, Box<dyn Any>> = HashMap::new();
        values.insert("rt_mihomosh_version", Box::new(env!("CARGO_PKG_VERSION")));
        values.insert("rt_default_ua", Box::new(DEFAULT_USER_AGENT));

        self.render_with_values(&values).map_err(Into::into)
    }
}

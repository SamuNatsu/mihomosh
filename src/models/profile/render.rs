use eyre::{Context, Result};
use serde_json::{Value, json};
use tinytemplate::TinyTemplate;

use crate::templates;

use super::entity::*;

impl Model {
    pub fn render(&self) -> Result<String> {
        // Setup template engine
        let mut tt = TinyTemplate::new();
        tt.add_template("profile", templates::PROFILE)
            .wrap_err("failed to add rendering template")?;
        tt.set_default_formatter(&|value, output| {
            if let Value::String(str) = &value {
                output.push_str(&serde_json::to_string(str)?);
                Ok(())
            } else {
                tinytemplate::format(value, output)
            }
        });

        // Create rendering context
        let mut ctx = self.clone();
        ctx.render_opts = Some(json!({
            "is-remote": self.r#type.is_remote(),
            "has-user-agent": self.r#type.is_remote() && self.user_agent.is_some(),
            "has-update-proxy": self.r#type.is_remote() && self.update_proxy.is_some(),
            "has-allow-invalid-certs": self.r#type.is_remote() && self.allow_invalid_certs.is_some(),
            "default-user-agent": DEFAULT_USER_AGENT,
        }));

        // Render profile information
        Ok(tt.render("profile", &ctx)?)
    }
}

use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, path::Path};

mod api;
mod headers;
mod writer;

#[derive(Deserialize, Debug)]
pub struct Config {
    identity: String,
    authorization: String,
    blade_auth: String,
    sites: HashMap<String, String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = toml::from_str(&read_to_string("config.toml")?)?;
    headers::set_identity(config.identity);
    headers::set_authorization(config.authorization);
    headers::set_blade_auth(config.blade_auth);
    for (dest, site_id) in config.sites {
        let result = api::list_all_students(site_id).await?;
        println!("{}: {:#?}", dest, result);
        writer::write_file(Path::new(&format!("{}.csv", dest)), result)?;
    }
    Ok(())
}

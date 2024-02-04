use std::{fs, path::Path};

use api::{build_router, Config};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let config_str =
        fs::read_to_string(Path::new("config.toml")).expect("Unable to read config file");
    let config: Config = toml::from_str(&config_str).expect("Unable to parse config file");

    let app = build_router(config, "./shuttle-dist/");
    Ok(app.into())
}

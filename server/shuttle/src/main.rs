#![warn(clippy::all, clippy::pedantic)]

use api::{build_router, Config};

use shuttle_runtime::{tokio::fs, tokio::sync::RwLock, CustomError};
use std::{path::Path, sync::Arc};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let config_str = fs::read_to_string(Path::new("config.toml"))
        .await
        .map_err(CustomError::new)?;
    let config: Config = toml::from_str(&config_str).map_err(CustomError::new)?;
    let app = build_router(Arc::new(RwLock::new(config)), "./shuttle-dist/");
    Ok(app.into())
}

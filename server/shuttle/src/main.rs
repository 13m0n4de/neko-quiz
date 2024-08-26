use std::{
    fs,
    path::{Path, PathBuf},
};

use api::{build_router, Config};
use shuttle_runtime::CustomError;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let config_str = fs::read_to_string(Path::new("config.toml")).map_err(CustomError::new)?;
    let config: Config = toml::from_str(&config_str).map_err(CustomError::new)?;
    let app = build_router(config, PathBuf::from("./shuttle-dist/"));
    Ok(app.into())
}

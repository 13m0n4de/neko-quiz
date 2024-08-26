use api::{build_router, Config};
use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[clap(name = "neko-quiz")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1:3000")]
    addr: SocketAddr,

    #[clap(short = 'c', long = "config", default_value = "config.toml")]
    config: PathBuf,

    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: PathBuf,
}

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> AppResult<()> {
    let opt = Opt::parse();

    setup_logging(&opt.log_level);

    let config = load_config(&opt.config)?;

    let app = build_router(config, opt.static_dir);

    start_server(opt.addr, app).await?;

    Ok(())
}

fn setup_logging(log_level: &str) {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", log_level));
    }
    tracing_subscriber::fmt::init();
}

fn load_config(config_path: &PathBuf) -> AppResult<Config> {
    let config_str = std::fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

async fn start_server(addr: SocketAddr, app: axum::Router) -> AppResult<()> {
    tracing::info!("listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

use api::{build_router, Config};
use clap::Parser;
use std::{fs, net::Ipv4Addr, path::Path, str::FromStr};
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[clap(name = "neko-quiz")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,

    #[clap(short = 'p', long = "port", default_value = "3000")]
    port: u16,

    #[clap(short = 'c', long = "config", default_value = "config.toml")]
    config: String,

    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }
    tracing_subscriber::fmt::init();

    let config_str =
        fs::read_to_string(Path::new(&opt.config)).expect("Unable to read config file");
    let config: Config = toml::from_str(&config_str).expect("Unable to parse config file");

    let app = build_router(config, &opt.static_dir);

    let addr = (
        Ipv4Addr::from_str(&opt.addr).unwrap_or(Ipv4Addr::LOCALHOST),
        opt.port,
    );

    tracing::info!("listening on http://{}:{}", addr.0, addr.1);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Unable to bind socket address");

    axum::serve(listener, app)
        .await
        .expect("Unable to start server");
}

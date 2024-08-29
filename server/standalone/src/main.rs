#![warn(clippy::all, clippy::pedantic)]

use api::{build_router, Config};

use clap::Parser;
use notify::{Event, RecursiveMode, Watcher};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::{
    net::TcpListener,
    signal,
    sync::{mpsc, watch, RwLock},
};
use tower_livereload::LiveReloadLayer;
use tracing::{debug, error, info};

#[derive(Parser, Debug)]
#[command(name = "neko-quiz")]
struct Opt {
    #[arg(short = 'l', long = "log", default_value = "info")]
    log_level: String,

    #[arg(short = 'a', long = "addr", default_value = "127.0.0.1:3000")]
    addr: SocketAddr,

    #[arg(short = 'c', long = "config", default_value = "config.toml")]
    config: PathBuf,

    #[arg(long = "static-dir", default_value = "./dist")]
    static_dir: PathBuf,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("Failed to bind to address {addr}")]
    Bind { addr: std::net::SocketAddr },

    #[error("Watcher error: {0}")]
    Watcher(#[from] notify::Error),
}

type AppResult<T> = Result<T, AppError>;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => info!("NekoQuiz server shut down successfully"),
        Err(e) => error!("{e}"),
    }
}

async fn run() -> AppResult<()> {
    let opt = Opt::parse();

    setup_logging(&opt.log_level);
    info!("Starting NekoQuiz server");

    let config = Arc::new(RwLock::new(load_config(&opt.config).await?));

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let app = build_router(Arc::clone(&config), opt.static_dir).layer(livereload);

    let (fs_event_tx, fs_event_rx) = mpsc::channel(100);
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Err(e) = fs_event_tx.blocking_send(res) {
            error!("Error sending file change event: {e}");
        }
    })?;
    watcher.watch(&opt.config, RecursiveMode::NonRecursive)?;

    let (cancel_tx, cancel_rx) = watch::channel(());

    let listener = TcpListener::bind(opt.addr)
        .await
        .map_err(|_| AppError::Bind { addr: opt.addr })?;
    info!("listening on http://{}", opt.addr);

    let server =
        axum::serve(listener, app).with_graceful_shutdown(shutdown_signal(cancel_rx.clone()));
    let watcher = watch_files(
        opt.config,
        Arc::clone(&config),
        reloader,
        fs_event_rx,
        cancel_rx.clone(),
    );

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                error!("Server error: {e}");
            }
        }
        result = watcher => {
            if let Err(e) = result {
                error!("File watcher error: {e}");
            }
        }
    }

    let _ = cancel_tx.send(());

    Ok(())
}

fn setup_logging(log_level: &str) {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{log_level},hyper=info,mio=info"));
    }
    tracing_subscriber::fmt::init();
}

async fn load_config(config_path: &PathBuf) -> AppResult<Config> {
    let config_str = tokio::fs::read_to_string(config_path).await?;
    let config: Config = toml::from_str(&config_str)?;
    info!("Configuration loaded successfully");
    Ok(config)
}

async fn watch_files(
    config_path: PathBuf,
    config: Arc<RwLock<Config>>,
    reloader: tower_livereload::Reloader,
    mut fs_event_rx: mpsc::Receiver<Result<Event, notify::Error>>,
    mut cancel_rx: watch::Receiver<()>,
) -> AppResult<()> {
    let watch_task = async move {
        while let Some(res) = fs_event_rx.recv().await {
            match res {
                Ok(event) if event.kind.is_modify() => {
                    debug!("Config file change detected");
                    match load_config(&config_path).await {
                        Ok(new_config) => {
                            *config.write().await = new_config;
                            reloader.reload();
                        }
                        Err(e) => error!("Failed to reload config: {e}"),
                    }
                }
                Err(e) => error!("Watch error: {e}"),
                _ => {}
            };
        }
    };

    tokio::select! {
        () = watch_task => {},
        _ = cancel_rx.changed() => {
            info!("Shutting down config watcher");
        }
    }

    Ok(())
}

async fn shutdown_signal(mut cancel_rx: watch::Receiver<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => debug!("Received Ctrl+C signal"),
        () = terminate => debug!("Received terminate signal"),
        _ = cancel_rx.changed() => {}
    }

    info!("Shutdown signal received");
}

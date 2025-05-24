#[cfg(feature = "ssr")]
use neko_quiz::models::config::Config;
#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("Failed to bind to address {addr}")]
    Bind { addr: std::net::SocketAddr },
}

#[cfg(feature = "ssr")]
type AppResult<T> = Result<T, AppError>;

#[cfg(feature = "ssr")]
async fn load_config(config_path: &str) -> AppResult<Config> {
    let config_str = tokio::fs::read_to_string(config_path).await?;
    let mut config: Config = toml::from_str(&config_str)?;
    for (index, question) in config.questions.iter_mut().enumerate() {
        question.id = format!("q{}", index + 1);
    }
    Ok(config)
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> AppResult<()> {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::{provide_context, *};
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use neko_quiz::app::{App, shell};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let routes = generate_route_list(App);

    use std::sync::Arc;

    let quiz_config_path = std::env::var("QUIZ_CONFIG").unwrap_or("config.toml".to_string());
    let config = Arc::new(load_config(&quiz_config_path).await?);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(config.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|_| AppError::Bind { addr })?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

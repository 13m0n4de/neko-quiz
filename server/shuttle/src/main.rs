use api::build_router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = build_router("config.json", "./shuttle-dist/");
    Ok(app.into())
}
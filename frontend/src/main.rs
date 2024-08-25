mod api;
mod app;
mod components;
mod models;
mod state;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

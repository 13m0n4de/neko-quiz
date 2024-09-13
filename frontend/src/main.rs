#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::enum_variant_names)]

mod api;
mod app;
mod components;
mod error;
mod models;
mod state;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

mod app;
mod components;
mod context;
mod game;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

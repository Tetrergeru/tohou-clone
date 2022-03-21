mod app;
mod audio;
mod enemies;
mod geometry;
mod level;
mod textures;
mod world;
mod download;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}

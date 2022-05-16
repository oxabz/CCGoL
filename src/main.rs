mod app;
mod gridcanvas;
mod gameoflife;
mod config_panel;
mod kernel_editor;
mod play_controls;

use app::App;

fn main() {
    wasm_logger::init(Default::default());
    yew::start_app::<App>();
}

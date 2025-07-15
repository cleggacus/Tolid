use crate::app::App;

mod screen_buffer;
mod app;
mod renderer;
mod events;
mod component;

fn main() {
    App::new().run().unwrap();
}

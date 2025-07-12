use crate::app::App;

mod screen_buffer;
mod app;
mod renderer;
mod events;

fn main() {
    App::new()
        .run();
}

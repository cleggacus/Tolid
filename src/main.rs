use crate::component::{Bouncer, Component, Root};

mod screen_buffer;
mod app;
mod renderer;
mod events;
mod component;

fn Bounce() -> impl Component {
    let bouncer = Bouncer::new();
    return bouncer;
}

fn App() -> impl Component {
    let mut root = Root::new();
    root.set_child(Box::new(Bounce()));
    return root;
}

fn main() {
    app::App::new(App)
        .run()
        .unwrap();
}

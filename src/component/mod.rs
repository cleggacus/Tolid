use crate::renderer::Renderer;

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}

pub trait Component: Renderable {}

pub struct Root {}

impl Root {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Root {}

impl Renderable for Root {
    fn render(&self, renderer: &mut crate::renderer::Renderer) {
        let (w, h) = renderer.size();
        renderer.draw_box(0, 0, w, h);
    }
}


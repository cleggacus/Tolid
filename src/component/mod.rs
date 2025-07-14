use crate::renderer::Renderer;

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);
}

pub trait Component: Renderable {}

pub struct Root {
    child: Option<Box<dyn Component>>,
}

impl Root {
    pub fn new() -> Self {
        Self {
            child: None
        }
    }

    pub fn set_child(&mut self, child: Box<dyn Component>) {
        self.child = Some(child);
    }
}

impl Component for Root {
}

impl Renderable for Root {
    fn render(&self, renderer: &mut crate::renderer::Renderer) {

        let render_context = renderer.current_render_context();
        let width = render_context.width;
        let height = render_context.height;

        renderer.draw_box(0, 0, width, height);

        renderer.push_render_context(1, 1, width-1, height-1);

        if let Some(child) = &self.child {
            child.render(renderer);
        }

        renderer.pop_render_context();
    }
}


pub struct Bouncer {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Bouncer {
    pub fn new() -> Self {
        Self {
            width: 20,
            height: 7,
            x: 0,
            y: 0,
        }
    }
}

impl Component for Bouncer {}

impl Renderable for Bouncer {
    fn render(&self, renderer: &mut crate::renderer::Renderer) {
        renderer.draw_box(
            self.x,
            self.y, 
            self.width, 
            self.height
        );
    }
}

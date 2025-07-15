use crate::renderer::{RenderContext, Renderer};

pub trait Renderable {
    fn render(&mut self, renderer: &mut Renderer);
}

pub trait Updatable {
    fn update(&mut self);
}

pub trait Component: Renderable + Updatable {}

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

impl Updatable for Root {
    fn update(&mut self) {
        if let Some(child) = &mut self.child {
            child.update();
        }
    }
}

impl Renderable for Root {
    fn render(&mut self, renderer: &mut crate::renderer::Renderer) {

        let render_context = renderer.current_render_context();
        let width = render_context.width;
        let height = render_context.height;

        renderer.draw_box(0, 0, width, height);

        renderer.push_render_context(1, 1, width-1, height-1);

        if let Some(child) = &mut self.child {
            child.render(renderer);
        }

        renderer.pop_render_context();
    }
}


pub struct Bouncer {
    render_context: Option<RenderContext>,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    vx: i32,
    vy: i32,
}

impl Bouncer {
    pub fn new() -> Self {
        Self {
            render_context: None,
            width: 20,
            height: 7,
            x: 0,
            y: 0,
            vx: 1,
            vy: 1,
        }
    }
}

impl Component for Bouncer {}

impl Updatable for Bouncer {
    fn update(&mut self) {
        if let Some(render_context) = &self.render_context {
            if self.x+self.width == render_context.width - 2 && self.vx > 0 {
                self.vx = 1;
            } else if self.x+self.width >= render_context.width - 1 {
                self.vx = -2;
            } else if self.x == 1 && self.vx < 0 {
                self.vx = -1;
            } else if self.x == 0 {
                self.vx = 2;
            }

            if self.y+self.height >= render_context.height - 1 {
                self.vy = -1;
            } else if self.y == 0 {
                self.vy = 1;
            }

            if self.vy > 0 {
                self.y += self.vy as usize;
            } else if self.vy < 0 {
                self.y -= (-self.vy) as usize;
            }

            if self.vx > 0 {
                self.x += self.vx as usize;
            } else if self.vx < 0 {
                self.x -= (-self.vx) as usize;
            }
        }
    }
}

impl Renderable for Bouncer {
    fn render(&mut self, renderer: &mut crate::renderer::Renderer) {
        self.render_context = Some(*renderer.current_render_context());

        renderer.draw_box(
            self.x as usize,
            self.y as usize, 
            self.width, 
            self.height
        );
    }
}

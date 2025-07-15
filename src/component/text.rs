use crossterm::style::Stylize;

use crate::{component::{Component, ComponentEvent, Rect}, renderer::Renderer};

pub struct Text {
    bounds: Rect,
    value: String,
    on_click: Option<Box<dyn Fn(&mut Text)>>,
}

impl Text {
    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn on_click<F>(mut self, on_click: F) -> Self 
    where
        F: Fn(&mut Text) + 'static,
    {
        self.on_click = Some(Box::new(on_click));
        self
    }
}

impl Component for Text {
    fn render(&mut self, renderer: &mut Renderer) {
        let render_context = renderer.current_render_context();

        self.bounds = Rect {
            x: render_context.x,
            y: render_context.y,
            width: render_context.width,
            height: render_context.height,
        };

        for (i, c) in self.value.chars().enumerate() {
            renderer.set(i, 0, c.stylize());
        }
    }

    fn propagate_event(&mut self, event: &ComponentEvent) {
        match event {
            ComponentEvent::OnClick(x, y) => {
                if 
                    *x < self.bounds.x || *x >= self.bounds.x + self.bounds.width ||
                    *y < self.bounds.y || *y >= self.bounds.y + self.bounds.height 
                {
                    return;
                }

                if let Some(on_click) = self.on_click.take() {
                    on_click(self);
                    self.on_click = Some(on_click);
                }
            }
        }
    }
}

pub fn text() -> Text {
    Text {
        bounds: Rect::default(),
        value: "".into(),
        on_click: None,
    }
}

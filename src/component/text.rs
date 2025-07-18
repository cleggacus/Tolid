use crossterm::style::Stylize;

use crate::{component::{Component, ComponentEvent, ComponentValue, IntoComponentValue, Rect}, renderer::Renderer};

pub struct Text {
    bounds: Rect,
    value: ComponentValue<String>,
    on_click: Option<Box<dyn FnMut(&mut Text)>>,
}

impl Text {
    pub fn value<T: IntoComponentValue<String>>(mut self, value: T) -> Self {
        self.value = value.into_component_value();
        self
    }

    pub fn set_value<T: IntoComponentValue<String>>(&mut self, value: T) {
        self.value = value.into_component_value();
    }

    pub fn on_click<F>(mut self, on_click: F) -> Self 
    where
        F: FnMut(&mut Text) + 'static,
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

        let value = match &self.value {
            ComponentValue::Static(value) => value,
            ComponentValue::Dynamic(value_fn) => &value_fn(),
        };

        for (i, c) in value.chars().enumerate() {
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

                if let Some(mut on_click) = self.on_click.take() {
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
        value: ComponentValue::Static("".into()),
        on_click: None,
    }
}

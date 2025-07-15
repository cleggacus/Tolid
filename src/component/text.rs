use crossterm::style::Stylize;

use crate::{component::Component, renderer::Renderer};

pub struct Text {
    value: String
}

impl Text {
    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }
}

impl Component for Text {
    fn render(&self, renderer: &mut Renderer) {
        for (i, c) in self.value.chars().enumerate() {
            renderer.set(i, 0, c.stylize());
        }
    }
}

pub fn text() -> Text {
    Text {
        value: "".into()
    }
}

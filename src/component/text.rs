use crossterm::style::Stylize;

use crate::{component::{Component, ComponentEvent, ComponentValue, Rect}, prelude::{Direction, ResolvedStackWidth, Sides, StackComponent, StackWidth}, renderer::Renderer, state::StateContext};

pub struct TextComponent {
    bounds: Rect,
    value: ComponentValue<String>,
    on_click: Option<Box<dyn FnMut()>>,
    width: StackWidth,
    padding: Sides,
}

impl Component for TextComponent {
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
            renderer.set(i + self.padding.3, self.padding.0, c.stylize());
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
                    on_click();
                    self.on_click = Some(on_click);
                }
            }
        }
    }

    fn resolve_stack_width(&self, stack: &StackComponent) -> ResolvedStackWidth {
        match self.width {
            StackWidth::Content => {
                let direction = stack.get_direction();

                let val = match direction {
                    Direction::Row => 1 + self.padding.0 + self.padding.2,
                    Direction::Column => {
                        let value = match &self.value {
                            ComponentValue::Static(value) => value,
                            ComponentValue::Dynamic(value_fn) => &value_fn(),
                        };

                        value.len() + self.padding.1 + self.padding.3
                    }
                };

                ResolvedStackWidth::Exact(val)
            },
            StackWidth::Flex(val) => ResolvedStackWidth::Flex(val),
            StackWidth::Exact(val) => ResolvedStackWidth::Exact(val),
        }
    }
}

#[derive(Default)]
pub struct TextProps {
    pub value: ComponentValue<String>,
    pub on_click: Option<Box<dyn FnMut()>>,
    pub width: StackWidth,
    pub padding: Sides,
}

#[allow(non_snake_case)]
pub fn Text(_ctx: StateContext, props: TextProps) -> TextComponent {
    TextComponent {
        bounds: Rect::default(),
        value: props.value,
        on_click: props.on_click,
        width: props.width,
        padding: props.padding,
    }
}

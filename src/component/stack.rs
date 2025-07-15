use std::cmp::Ordering;

use crate::{component::{Component, ComponentEvent, Rect}, renderer::Renderer};

pub enum Direction {
    Row,
    Column
}

pub enum StackWidth {
    Flex(usize),
    Exact(usize),
}

pub struct StackChild {
    pub component: Box<dyn Component>,
    pub width: StackWidth,
}

pub struct Stack {
    bounds: Rect,
    border: bool,
    direction: Direction,
    children: Vec<StackChild>
}

impl Stack {
    pub fn add_child<T: Component + 'static>(mut self, width: StackWidth, child: T) -> Self {
        self.children.push(StackChild { 
            component: Box::new(child), 
            width
        });

        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    fn calc_render_widths(&self, renderer: &mut Renderer) -> Vec<usize> {
        let mut flex_total: usize = 0;
        let mut flex_count: usize = 0;

        let mut exact_count: usize = 0;

        for child in &self.children {
            match child.width {
                StackWidth::Flex(val) => {
                    flex_total += val;
                    flex_count += 1;
                },
                StackWidth::Exact(_) => {
                    exact_count += 1;
                },
            }
        }

        let render_context = renderer.current_render_context();

        let total_potential_width = match self.direction {
            Direction::Row => render_context.height,
            Direction::Column => render_context.width,
        };

        let total_flex_width = total_potential_width - exact_count;

        let mut widths = Vec::<usize>::with_capacity(self.children.len());
        let mut flex_remainders = Vec::<(f32, usize)>::with_capacity(flex_count);
        let mut total_width: usize = 0;

        for child in &self.children {
            match child.width {
                StackWidth::Flex(val) => {
                    let percent_width = val as f32 / flex_total as f32;
                    let sub_pixel_width = percent_width * total_flex_width as f32;
                    let floored_width = sub_pixel_width.floor();
                    let remainder = sub_pixel_width - floored_width;

                    flex_remainders.push((remainder, widths.len()));
                    widths.push(floored_width as usize);
                },
                StackWidth::Exact(val) => {
                    widths.push(val);
                },
            }

            total_width += widths.last().copied().unwrap_or(0);
        }

        flex_remainders.sort_by(|a, b| { b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal) });

        let mut remaining = total_potential_width - total_width ;

        while remaining > 0 {
            let max = flex_remainders.pop();

            if let Some((_, i)) = max {
                widths[i] += 1;
                remaining -= 1;
            } else {
                break;
            }
        }

        widths
    }

    fn render_children(&mut self, renderer: &mut Renderer) {
        let widths = self.calc_render_widths(renderer);

        if widths.len() != self.children.len() {
            return;
        }

        let render_context = renderer.current_render_context();
        let width = render_context.width;
        let height = render_context.height;

        let mut offset: usize = 0;

        for i in 0..self.children.len() {
            let (new_x, new_y, new_width, new_height) = match self.direction {
                Direction::Row => (0, offset, width, widths[i]),
                Direction::Column => (offset, 0, widths[i], height),
            };

            renderer.push_relative_render_context(new_x, new_y, new_width, new_height);
            self.children[i].component.render(renderer);
            renderer.pop_render_context();

            offset += widths[i];
        }
    }
}

impl Component for Stack {
    fn render(&mut self, renderer: &mut Renderer) {
        let render_context = renderer.current_render_context();

        self.bounds = Rect {
            x: render_context.x,
            y: render_context.y,
            width: render_context.width,
            height: render_context.height,
        };

        let width = render_context.width;
        let height = render_context.height;

        if width == 0 && height == 0 {
            return;
        }

        if self.border {
            if width < 2 && height < 2 {
                return;
            }

            renderer.draw_box(0, 0, width, height);
            renderer.push_relative_render_context(1, 1, width - 2, height - 2);
        }

        self.render_children(renderer);

        if self.border {
            renderer.pop_render_context();
        }
    }

    fn propagate_event(&mut self, event: &ComponentEvent) {
        for child in &mut self.children {
            child.component.propagate_event(event);
        }
    }
}

pub fn stack() -> Stack {
    Stack {
        bounds: Rect::default(),
        border: false,
        direction: Direction::Row,
        children: vec![]
    }
}

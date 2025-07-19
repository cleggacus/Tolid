use std::cmp::Ordering;

use crate::{component::{Component, ComponentEvent, Rect}, prelude::Sides, renderer::Renderer, state::StateContext};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default] Row,
    Column
}

#[derive(Default, Copy, Clone)]
pub enum StackWidth {
    #[default] Content,
    Flex(usize),
    Exact(usize),
}

#[derive(Copy, Clone)]
pub enum ResolvedStackWidth {
    Flex(usize),
    Exact(usize),
}

#[derive(Default, Copy, Clone)]
pub enum StackAlign {
    #[default] Start,
    Center,
    End,
}

enum WidthSegment {
    Child(usize, usize),
    Filler(usize),
}

#[derive(Default)]
pub struct StackProps {
    pub border: bool,
    pub direction: Direction,
    pub children: Vec<Box<dyn Component>>,
    pub on_click: Option<Box<dyn FnMut()>>,
    pub width: StackWidth,
    pub align: StackAlign,
    pub padding: Sides,
}

pub struct StackComponent {
    bounds: Rect,
    props: StackProps,
}

impl StackComponent {
    pub fn get_border(&self) -> bool {
        self.props.border
    }

    pub fn get_direction(&self) -> Direction {
        self.props.direction
    }

    fn calc_render_widths(&self, renderer: &mut Renderer) -> Vec<WidthSegment> {
        let mut flex_total: usize = 0;
        let mut flex_count: usize = 0;

        let mut exact_total: usize = 0;

        for child in &self.props.children {
            let width = child.resolve_stack_width(self);

            match width {
                ResolvedStackWidth::Flex(val) => {
                    flex_total += val;
                    flex_count += 1;
                },
                ResolvedStackWidth::Exact(val) => {
                    exact_total += val;
                },
            }
        }

        let render_context = renderer.current_render_context();

        let total_potential_width = match self.props.direction {
            Direction::Row => render_context.height,
            Direction::Column => render_context.width,
        };

        let total_flex_width = total_potential_width - exact_total.min(total_potential_width);

        let mut widths = Vec::<WidthSegment>::new();
        let mut flex_remainders = Vec::<(f32, usize)>::new();
        let mut total_width: usize = 0;

        let mut children: Vec<Option<&Box<dyn Component>>> = vec![];

        let has_flex_children = flex_count > 0;

        if !has_flex_children {
            match self.props.align {
                StackAlign::Center |
                StackAlign::End => {
                    flex_total += 1;
                    children.push(None);
                },
                _ => {}
            };
        }

        self.props.children.iter()
            .for_each(|child| children.push(Some(child)));

        if !has_flex_children {
            match self.props.align {
                StackAlign::Center |
                StackAlign::Start => {
                    flex_total += 1;
                    children.push(None);
                },
                _ => {}
            };
        }

        let mut i = 0;

        for child in &children {
            let width = if let Some(child) = child {
                child.resolve_stack_width(self)
            } else {
                ResolvedStackWidth::Flex(1)
            };

            let width_amount = match width {
                ResolvedStackWidth::Flex(val) => {
                    let percent_width = val as f32 / flex_total as f32;
                    let sub_pixel_width = percent_width * total_flex_width as f32;
                    let floored_width = sub_pixel_width.floor();
                    let remainder = sub_pixel_width - floored_width.min(sub_pixel_width);

                    flex_remainders.push((remainder, widths.len()));
                    floored_width as usize
                },
                ResolvedStackWidth::Exact(val) => val,
            };

            widths.push(
                if let Some(_) = child {
                    i += 1;
                    WidthSegment::Child(width_amount, i-1)
                } else {
                    WidthSegment::Filler(width_amount)
                }
            );

            total_width += width_amount;
        }

        flex_remainders.sort_by(|a, b| { b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal) });

        let mut remaining = total_potential_width - total_width.min(total_potential_width);

        while remaining > 0 {
            let max = flex_remainders.pop();

            if let Some((_, i)) = max {
                match &mut widths[i] {
                    WidthSegment::Child(val, _) => *val += 1,
                    WidthSegment::Filler(val) => *val += 1,
                }

                remaining -= 1;
            } else {
                break;
            }
        }

        widths
    }

    fn render_children(&mut self, renderer: &mut Renderer) {
        let widths = self.calc_render_widths(renderer);

        let render_context = renderer.current_render_context();
        let width = render_context.width;
        let height = render_context.height;

        let mut offset: usize = 0;

        for width_segment in widths {
            let amount = match width_segment {
                WidthSegment::Child(amount, i) => {
                    let (new_x, new_y, new_width, new_height) = match self.props.direction {
                        Direction::Row => (0, offset, width, amount),
                        Direction::Column => (offset, 0, amount, height),
                    };

                    renderer.push_relative_render_context(new_x, new_y, new_width, new_height);
                    self.props.children[i].render(renderer);
                    renderer.pop_render_context();

                    amount
                },
                WidthSegment::Filler(amount) => amount,
            };

            offset += amount;
        }
    }
}

impl Component for StackComponent {
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

        let (mut x, mut y, mut width, mut height) = if self.props.border {
            renderer.draw_box(0, 0, width, height);
            (1, 1, width - 2.min(width), height - 2.min(height))
        } else {
            (0, 0, width, height)
        };

        x += self.props.padding.3;
        y += self.props.padding.0;
        width -= (self.props.padding.3 + self.props.padding.1).min(width);
        height -= (self.props.padding.0 + self.props.padding.2).min(width);

        renderer.push_relative_render_context(x, y, width, height);

        self.render_children(renderer);

        renderer.pop_render_context();
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

                if let Some(mut on_click) = self.props.on_click.take() {
                    on_click();
                    self.props.on_click = Some(on_click);
                }
            }
        }

        for child in &mut self.props.children {
            child.propagate_event(event);
        }
    }

    fn resolve_stack_width(&self, stack: &StackComponent) -> ResolvedStackWidth {
        match self.props.width {
            StackWidth::Content => {
                let mut total_width = if self.props.border {
                    2
                } else {
                    0
                };

                total_width += match stack.get_direction() {
                    Direction::Row => self.props.padding.0 + self.props.padding.2,
                    Direction::Column => self.props.padding.1 + self.props.padding.3,
                };

                if self.get_direction() != stack.get_direction() {
                    let mut max = 0;

                    for child in &self.props.children {
                        if let ResolvedStackWidth::Exact(val) = child.resolve_stack_width(stack) {
                            max = max.max(val);
                        }
                    }

                    total_width += max;
                } else {
                    for child in &self.props.children {
                        if let ResolvedStackWidth::Exact(val) = child.resolve_stack_width(stack) {
                            total_width += val;
                        }
                    }
                }

                ResolvedStackWidth::Exact(total_width)
            },
            StackWidth::Flex(val) => ResolvedStackWidth::Flex(val),
            StackWidth::Exact(val) => ResolvedStackWidth::Exact(val),
        }
    }
}

#[allow(non_snake_case)]
pub fn Stack(_ctx: StateContext, props: StackProps) -> StackComponent {
    StackComponent {
        bounds: Rect::default(),
        props
    }
}

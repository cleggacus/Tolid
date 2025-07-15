pub mod stack;
pub mod text;

use std::cmp::Ordering;

use crossterm::style::Stylize;

use crate::{renderer::Renderer};

pub trait Component {
    fn render(&self, renderer: &mut Renderer);
}

// pub trait Renderable {
//     fn render(&mut self, renderer: &mut Renderer);
// }
// 
// pub trait Component: Renderable {}
// 
// pub struct Root {
//     children: Vec<Box<dyn Component>>,
// }
// 
// impl Root {
//     pub fn new() -> Self {
//         Self {
//             children: vec![]
//         }
//     }
// 
//     pub fn add_child<C, F>(&mut self, child_fn: F)
//     where
//         C: Component + 'static,
//         F: Fn() -> C,
//     {
//         let child = child_fn();
//         self.children.push(Box::new(child));
//     }
// }
// 
// impl Component for Root {
// }
// 
// impl Renderable for Root {
//     fn render(&mut self, renderer: &mut crate::renderer::Renderer) {
// 
//         let render_context = renderer.current_render_context();
//         let width = render_context.width;
//         let height = render_context.height;
// 
//         renderer.draw_box(0, 0, width, height);
// 
//         renderer.push_render_context(1, 1, width-1, height-1);
// 
//         for child in &mut self.children {
//             child.render(renderer);
//         }
// 
//         renderer.pop_render_context();
//     }
// }
// 
// 
// pub struct Rect {
//     width: usize,
//     height: usize,
//     x: usize,
//     y: usize,
//     vx: i32,
//     vy: i32,
// }
// 
// impl Rect {
//     pub fn new(x: usize, y: usize) -> Self {
//         Self {
//             width: 20,
//             height: 7,
//             x,
//             y,
//             vx: 1,
//             vy: 1,
//         }
//     }
// }
// 
// impl Component for Rect {}
// 
// impl Renderable for Rect {
//     fn render(&mut self, renderer: &mut crate::renderer::Renderer) {
//         renderer.draw_box(
//             self.x as usize,
//             self.y as usize, 
//             self.width, 
//             self.height
//         );
//     }
// }

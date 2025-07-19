pub mod stack;
pub mod text;

use crate::{renderer::Renderer};

pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

pub trait Component {
    fn propagate_event(&mut self, event: &ComponentEvent);
    fn render(&mut self, renderer: &mut Renderer);
}

#[derive(Debug, Clone)]
pub enum ComponentEvent {
    OnClick(usize, usize),
}

pub enum ComponentValue<T> {
    Static(T),
    Dynamic(Box<dyn Fn() -> T>),
}

impl<T: Default> Default for ComponentValue<T> {
    fn default() -> Self {
        Self::Static(Default::default())
    }
}

pub trait IntoComponentValue<T> {
    fn into_component_value(self) -> ComponentValue<T>;
}


macro_rules! impl_into_component_value_fn {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<F> IntoComponentValue<$ty> for F
            where
                F: Fn() -> $ty + 'static,
            {
                fn into_component_value(self) -> ComponentValue<$ty> {
                    ComponentValue::Dynamic(Box::new(self))
                }
            }

            impl IntoComponentValue<$ty> for $ty {
                fn into_component_value(self) -> ComponentValue<$ty> {
                    ComponentValue::Static(self)
                }
            }
        )*
    };
}

impl_into_component_value_fn!(
    String,
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    bool,
    f32, f64,
    char,
);

impl IntoComponentValue<String> for &str {
    fn into_component_value(self) -> ComponentValue<String> {
        ComponentValue::Static(self.into())
    }
}

#[macro_export]
macro_rules! cm {
    ($var:ident || $body:block) => {
        {
            let $var = $var.clone();
            move || $body
        }
    };
    ($var:ident || $expr:expr) => {{
        let $var = $var.clone();
        move || $expr
    }};

    // Multi idents
    ([$($var:ident),+] || $body:block) => {{
        $(let $var = $var.clone();)+
        move || $body
    }};

    ([$($var:ident),+] || $expr:expr) => {{
        $(let $var = $var.clone();)+
        move || $expr
    }};
}

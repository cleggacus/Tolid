use crate::prelude::*;
use crate::marcos::component;

#[component]
pub fn Center(direction: Direction, border: bool, children: Vec<Box<dyn Component>>) -> impl Component {
    match direction {
        Direction::Row => ui! {
            <Stack direction={Direction::Column} align={StackAlign::Center} border={border} >
                <Stack direction={Direction::Row} align={StackAlign::Center}>
                    {children}
                </Stack>
            </Stack>
        },
        Direction::Column => ui! {
            <Stack direction={Direction::Row} align={StackAlign::Center} border={border} >
                <Stack direction={Direction::Column} align={StackAlign::Center}>
                    {children}
                </Stack>
            </Stack>
        }
    }
}

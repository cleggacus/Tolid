use crate::prelude::*;
use crate::marcos::component;

#[component]
pub fn Button(
    border: bool, 
    width: StackWidth, 
    value: String, 
    on_click: Option<Box<dyn FnMut()>>,
    padding: Sides,
) -> impl Component {
    ui! {
        <Stack
            width={width}
            border={border}
            on_click={on_click}
            padding={padding}
        >
            <Text
                width={StackWidth::Content}
                value={value}
            />
        </Stack>
    }
}

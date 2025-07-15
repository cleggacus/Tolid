use crate::component::{stack::{stack, Direction, StackWidth}, text::text, Component};

mod screen_buffer;
mod app;
mod renderer;
mod events;
mod component;

fn t1() -> impl Component {
    stack()
        .border(true)
        .direction(Direction::Row)
        .add_child(
            StackWidth::Flex(2),
            stack()
                .border(true)
                .direction(Direction::Row)
        )
        .add_child(
            StackWidth::Exact(1),
            text()
                .value("Yasss".into())
                .on_click(|s| { 
                    s.set_value("Queeen".into()); 
                })
        )
        .add_child(
            StackWidth::Exact(1),
            text().value("Queeeen!!!".into())
        )
        .add_child(
            StackWidth::Flex(3),
            stack()
                .border(true)
                .direction(Direction::Row)
        )
}

fn t2() -> impl Component {
    stack()
        .border(false)
        .direction(Direction::Row)
        .add_child(
            StackWidth::Flex(2),
            stack()
                .border(true)
                .direction(Direction::Row)
        )
        .add_child(
            StackWidth::Exact(1),
            text().value("Ayyyyy".into())
        )
        .add_child(
            StackWidth::Exact(1),
            text().value("Yooooo".into())
        )
        .add_child(
            StackWidth::Flex(2),
            stack()
                .border(true)
                .direction(Direction::Row)
        )
}

fn root() -> impl Component {
    stack()
        .border(true)
        .direction(Direction::Column)
        .add_child(
            StackWidth::Flex(1),
            t1()
        )
        .add_child(
            StackWidth::Flex(2),
            t2()
        )
}

fn main() {
    app::App::new(root())
        .run()
        .unwrap();
}

// Goal!!!
// 
// fn TextInputComponent(ctx: Scope, set_text: WriteSignal<String>, text: ReadSignal<String>) -> Node {
//     input()
//         .placeholder("Type something...")
//         .value(move || text.get())
//         .on_change(move |new_val| {
//             set_text.set(new_val);
//         })
//         .build()
// }
// 
// fn DisplayComponent(ctx: Scope, text: ReadSignal<String>) -> Node {
//     create_effect(cx, move || {
//         notify!("Text changed to: {}", text.get());
//     });
// 
//     text()
//         .child(move || text.get())
//         .build()
// }
// 
// fn App(ctx: Scope) -> impl Component {
//     let (text, set_text) = create_signal(ctx, String::new());
// 
//     vstack()
//         .child(text().child("Sandbox!!!"))
//         .child(TextInputComponent(cx, set_text, text))
//         .child(DisplayComponent(cx, text))
//         .build()
// }
// 
// fn main() {
//     app::App::new(App)
//         .run()
//         .unwrap();
// }

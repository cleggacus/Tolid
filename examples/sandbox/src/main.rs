use tolid::{app::App, component::{stack::{stack, Direction, StackWidth}, text::text, Component}};

fn t1() -> impl Component {
    let mut toggle = false;

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
                .value("Click Me!!!".into())
                .on_click(move |this| { 
                    toggle = !toggle;

                    let text = if toggle {
                        "Yaasssss"
                    } else {
                        "Slaayyyy"
                    };

                    this.set_value(text.into()); 
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
    App::new(root())
        .run()
        .unwrap();
}

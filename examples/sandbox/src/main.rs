use tolid::{app::App, component::{stack::{stack, Direction, StackWidth}, text::text, Component}, state::{use_state, GetState, SetState}};

fn t1(counter: GetState<i64>) -> impl Component {
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
            text().value(move || format!("Counter: {}", counter.get()))
        )
        .add_child(
            StackWidth::Flex(3),
            stack()
                .border(true)
                .direction(Direction::Row)
        )
}

fn t2(set_counter: SetState<i64>) -> impl Component {
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
            text()
                .value("Increment")
                .on_click(move |_| set_counter.update(|counter| counter+1))
        )
        .add_child(
            StackWidth::Flex(2),
            stack()
                .border(true)
                .direction(Direction::Row)
        )
}

fn t3(set_counter: SetState<i64>) -> impl Component {
    stack()
        .border(false)
        .direction(Direction::Row)
        .add_child(
            StackWidth::Flex(3),
            stack()
                .border(true)
                .direction(Direction::Row)
        )
        .add_child(
            StackWidth::Exact(1),
            text()
                .value("Decrement")
                .on_click(move |_| set_counter.update(|counter| counter-1))
        )
        .add_child(
            StackWidth::Flex(2),
            stack()
                .border(true)
                .direction(Direction::Row)
        )
}

fn root() -> impl Component {
    let (counter, set_counter) = use_state::<i64>(0);

    stack()
        .border(true)
        .direction(Direction::Column)
        .add_child(
            StackWidth::Flex(1),
            t1(counter.clone())
        )
        .add_child(
            StackWidth::Flex(2),
            t2(set_counter.clone())
        )
        .add_child(
            StackWidth::Flex(1),
            t3(set_counter)
        )
}

fn main() {
    App::new(root())
        .run()
        .unwrap();
}

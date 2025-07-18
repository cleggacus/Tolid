use tolid::{app::App, component::{stack::{Direction, Stack, StackProps, StackWidth}, text::{Text, TextProps}, Component, ComponentValue}, marcos::{component, ui}, state::{use_state, GetState, SetState}};

#[component]
fn DisplayCounter(counter: GetState<i64>, title: String) -> impl Component {
    ui! {
        <Stack border={true} widths={vec![StackWidth::Exact(1), StackWidth::Flex(1), StackWidth::Exact(1)]} >
            <Text value={ComponentValue::Static(title)} />
            <Stack />
            <Text value={ComponentValue::Dynamic(Box::new(move || 
                format!("Value: {}", counter.get())
            ))} />
        </Stack>
    }
}

#[component]
fn Incrementer(set_counter: SetState<i64>) -> impl Component {
    ui! {
        <Stack border={true} widths={vec![StackWidth::Flex(1), StackWidth::Exact(1), StackWidth::Flex(1)]} >
            <Stack />
            <Text 
                value={ComponentValue::Static("Increment".into())} 
                on_click={Some(Box::new(move |_| {
                    set_counter.update(|counter| counter+1);
                }))} 
            />
            <Stack />
        </Stack>
    }
}

#[component]
fn Root() -> impl Component {
    let (counter, set_counter) = use_state::<i64>(0);

    ui! {
        <Stack border={true} direction={Direction::Column} widths={vec![StackWidth::Flex(1), StackWidth::Flex(2), StackWidth::Flex(1)]} >
            <DisplayCounter counter={counter.clone()} title={"Counter!!!".into()} />
            <Stack />
            <Incrementer set_counter={set_counter.clone()} />
        </Stack>
    }
}

fn main() {
    App::new(Root(Default::default()))
        .run()
        .unwrap();
}

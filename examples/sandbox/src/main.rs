use tolid::prelude::*;

#[component]
fn DisplayCounter(counter: GetState<i64>, label: String) -> impl Component {
    ui! {
        <Text value={move || format!("{}: {}", label, counter.get())} />
    }
}

#[component]
fn Incrementer(set_counter: SetState<i64>) -> impl Component {
    ui! {
        <Stack
            border={true} 
            widths={vec![StackWidth::Flex(1)]} 
            on_click={move |_| {
                set_counter.update(|counter| counter+1);
            }} 
        >
            <Text value={"Increment".into()}/>
        </Stack>
    }
}

#[component]
fn Root() -> impl Component {
    let (counter, set_counter) = use_state::<i64>(0);

    ui! {
        <Stack widths={vec![StackWidth::Flex(1), StackWidth::Exact(3)]} >
            <DisplayCounter counter={counter.clone()} label={"Counter".into()} />
            <Incrementer set_counter={set_counter.clone()} />
        </Stack>
    }
}

fn main() {
    run_app(Root);
}


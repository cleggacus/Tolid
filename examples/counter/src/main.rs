use tolid::prelude::*;

#[component]
fn App() -> impl Component {
    // Create counter state
    let (counter, set_counter) = use_state(ctx.clone(), 0_i64);

    let increment = move || set_counter.update(|counter| counter+1);
    let value = move || format!("Counter: {}", counter.get());

    // Return component with markup!!!
    ui! {
        <Stack border={true} widths={vec![StackWidth::Exact(1), StackWidth::Exact(1), StackWidth::Exact(1)]} >
            <Text value={value} />

            <Text 
                value={"Increment"}
                on_click={increment}
            />
        </Stack>
    }
}

fn main() {
    run_app(App);
}


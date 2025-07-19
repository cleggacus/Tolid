use tolid::prelude::*;

#[component]
fn App() -> impl Component {
    // Initialize counter state with custom save hook
    let (counter, set_counter) = use_state(ctx.clone(), 0_i64);

    let increment = cm!(set_counter || set_counter.update(|counter| counter+1));
    let decrement = cm!(set_counter || set_counter.update(|counter| counter-1));

    let value = cm!(counter || format!("Counter: {}", counter.get()));

    ui! {
        <Stack border={true} widths={vec![StackWidth::Exact(1), StackWidth::Exact(1), StackWidth::Exact(1)]} >
            <Text value={value} />

            <Text 
                value={"Increment"}
                on_click={increment}
            />

            <Text 
                value={"Decrement"}
                on_click={decrement}
            />
        </Stack>
    }
}

fn main() {
    run_app(App);
}


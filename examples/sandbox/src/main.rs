use std::{fmt::Display, str::FromStr};

use tolid::prelude::*;

fn use_save<T: FromStr + Clone + Display + 'static>(ctx: StateContext, initial: T) -> (GetState<T>, SetState<T>) {
    // Create counter state
    let (counter, set_counter) = use_state::<T>(ctx.clone(), initial);

    // Set initial state from file "save"
    use_effect(ctx.clone(), cm!(set_counter || 
        if let Ok(contents) = std::fs::read_to_string("save") {
            if let Ok(val) = contents.trim().parse::<T>() {
                set_counter.set(val);
            }
        }
    ));

    // When counter changes, write value to file "save"
    use_effect(ctx.clone(), cm!(counter || 
        std::fs::write("save", format!("{}", counter.get()))
            .expect("Failed to write to log")
    ));

    // return state to be used externally
    (counter, set_counter)
}


#[component]
fn App() -> impl Component {
    // Initialize counter state with custom save hook
    let (counter, set_counter) = use_save(ctx.clone(), 0_i64);

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


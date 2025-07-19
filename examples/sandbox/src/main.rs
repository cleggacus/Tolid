use std::{fmt::Display, str::FromStr};

use tolid::prelude::*;

fn use_save<T: FromStr + Clone + Display + 'static>(ctx: StateContext, initial: T) -> (GetState<T>, SetState<T>) {
    // Create counter state
    let (counter, set_counter) = use_state::<T>(ctx.clone(), initial);

    // Set initial state from file "save"
    use_effect(ctx.clone(), {
        let set_counter = set_counter.clone();

        move || {
            if let Ok(contents) = std::fs::read_to_string("save") {
                if let Ok(val) = contents.trim().parse::<T>() {
                    set_counter.set(val);
                }
            }
        }
    });

    // When counter changes, write value to file "save"
    use_effect(ctx.clone(), {
        let counter = counter.clone();

        move || {
            std::fs::write("save", format!("{}", counter.get()))
                .expect("Failed to write to log");
        }
    });

    // return state to be used externally
    (counter, set_counter)
}

#[component]
fn App() -> impl Component {
    // Initialize counter state with custom save hook
    let (counter, set_counter) = use_save(ctx.clone(), 0_i64);

    // Return component with markup!!!
    ui! {
        <Stack border={true} widths={vec![StackWidth::Exact(1), StackWidth::Exact(1), StackWidth::Exact(1)]} >
            <Text value={move || {
                // Display Counter 
                format!("Counter: {}", counter.get())
            }} />

            <Text 
                value={"Increment".into()}
                on_click={
                    move |_| {
                        // Increment Counter by 1
                        set_counter.update(|counter| counter+1);
                    }
                }
            />

            <Text 
                value={"Decrement".into()}
                on_click={move |_| {
                    // Decrement Counter by 1
                    set_counter.update(|counter| counter-1);
                }} 
            />
        </Stack>
    }
}

fn main() {
    run_app(App);
}


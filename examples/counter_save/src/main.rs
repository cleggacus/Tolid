use tolid::prelude::*;
use std::{fmt::Display, str::FromStr};

pub fn use_save<T: FromStr + Clone + Display + 'static>(ctx: StateContext, initial: T) -> (GetState<T>, SetState<T>) {
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
        <Center direction={Direction::Column} border={true}>
            <Button
                padding={(0, 1, 0, 1)}
                width={StackWidth::Content} 
                border={true}
                on:click={decrement}
                value={"<<".into()}
            />

            <Text padding={(1, 2, 1, 2)} value={value} />

            <Button
                padding={(0, 1, 0, 1)}
                width={StackWidth::Content} 
                border={true}
                on:click={increment}
                value={">>".into()}
            />
        </Center>
    }
}

fn main() {
    run_app(App);
}


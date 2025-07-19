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

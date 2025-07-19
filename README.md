<h1 align="center">Tolid - React style framework for TUI's</h1>
<p  align="center">
  <img width="100%" src="https://i.imgur.com/Ha9Gu49.gif"/>
</p>

## Counter Example

### Basic

Here's a simple counter example.
As you can see, it's very similar to React function components!

```rust
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

```

### Adding Decrement

As we begin using state across multiple closures, we often need to `clone` it to satisfy ownership and borrowing rules.

The `cm!` macro simplifies this by automatically cloning and moving both the state getter and setter into the closure, making the code cleaner.


```rust
// with cm!
let increment = cm!(set_counter || set_counter.update(|counter| counter+1));

// without cm!
let increment = {
    let set_counter = set_counter.clone();
    move || set_counter.update(|counter| counter+1))
};
```

Below is a full example using both increment and decrement functions:

```rust
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

```


### Adding custom Hooks

We can take this further by wrapping our counter logic in a custom save hook.

Using use_effect, similar to React, we can load the state from a file and save changes back to it automatically.

```rust
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

```

Now we can use this custom hook in our app:

```rust
#[component]
fn App() -> impl Component {
    // Initialize counter state with custom save hook
    let (counter, set_counter) = use_state(ctx.clone(), 0_i64);

    // ... rest of the code
}
```

<h1 align="center">Tolid - React style framework for TUI's</h1>
<p  align="center">
  <img width="100%" src="https://i.imgur.com/Ha9Gu49.gif"/>
</p>

## Counter Example
Here is a simple counter example. 

As you can see, it's very similar to React function components!!!
```rust
use tolid::prelude::*;

#[component]
fn Root() -> impl Component {
    // Create counter state
    let (counter, set_counter) = use_state::<i64>(0);

    // Return component with markup!!!
    ui! {
        <Stack border={true} widths={vec![StackWidth::Exact(1)]} >
            <Text value={move || {
                // Display Counter 
                format!("Counter: {}", counter.get())
            }} />

            <Text 
                value={"Increment".into()}
                on_click={move |_| {
                    // Increment Counter by 1
                    set_counter.update(|counter| counter+1);
                }} 
            />
        </Stack>
    }
}


fn main() {
    App::new(Counter(Default::default()))
        .run()
        .unwrap();
}
```

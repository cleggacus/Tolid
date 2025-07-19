use tolid::prelude::*;

#[component]
fn App() -> impl Component {
    // Create counter state
    let (counter, set_counter) = use_state(ctx.clone(), 0_i64);

    let increment = move || set_counter.update(|counter| counter+1);
    let value = move || format!("Counter: {}", counter.get());

    // Return component with markup!!!
    ui! {
        <Center border={true}>
            <Text value={value} />

            <Button
                border={true}
                on:click={increment}
                value={"Increment".into()}
            />
        </Center>
    }
}

fn main() {
    run_app(App);
}


use tolid::prelude::*;

#[component]
fn App() -> impl Component {
    // Create counter state
    let (counter, set_counter) = use_state(ctx.clone(), 0_i64);

    let increment = move || set_counter.update(|counter| counter+1);
    let value = move || format!("Counter: {}", counter.get());

    // Return component with markup!!!
    ui! {
        <Center direction={Direction::Column} border={true}>
            <Text padding={(1, 2, 1, 2)} value={value} />

            <Button
                padding={(0, 1, 0, 1)}
                width={StackWidth::Content} 
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


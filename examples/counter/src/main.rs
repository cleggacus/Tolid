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
            <Stack>
                <Stack width={StackWidth::Exact(1)} />
                <Text value={value} />
            </Stack>


            <Stack 
                direction={Direction::Column} 
                width={StackWidth::Content} 
                border={true}
                on_click={increment}
            >
                <Text 
                    width={StackWidth::Content}
                    value={"Increment"}
                />
            </Stack>
        </Center>
    }
}

fn main() {
    run_app(App);
}


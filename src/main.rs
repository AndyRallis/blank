use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            counter.set(*counter + 1)
        }
    };
    html! {<>
        <h1>{"Hello World!!"}</h1>
        <button {onclick}>{"+1"}</button>
        <p>{*counter}</p>
        </>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}

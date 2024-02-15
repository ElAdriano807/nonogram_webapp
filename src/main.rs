use yew::prelude::*;
use yew::virtual_dom::VNode;

fn main() {
    yew::Renderer::<App>::new().render();
}

struct Model {
    todos: Vec<String>,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        todos: vec!["Hello".to_string(), "World".to_string()],
    });

    let onclick = {
        let state = state.clone();
        println!("TEST");
        Callback::from(move |_| {
            let mut todos = state.todos.clone();
            todos.push("test".to_string());
            state.set(Model {
                todos
            })
        })
    };

    let ps: Vec<VNode> = state.todos.clone().into_iter().map(|todo| {
        html!{
            <p>{todo}</p>
        }
    }).collect();

    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            {ps}
        </div>
    }
}


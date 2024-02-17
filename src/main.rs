use log::info;
use web_sys::{Event, HtmlInputElement as InputElement, MouseEvent};
use yew::events::KeyboardEvent;
use yew::html::Scope;
use yew::{html, Component, Context, Html, TargetCast, Callback, Properties};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

struct State {
    todos: Vec<String>,
}

enum Message {
    Add(String),
    DeleteAll,
}

struct App {
    state: State,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let state = State {
            todos: vec![],
        };
        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Add(todo) => {
                info!("Receive Add({})", todo);
                let todo = todo.trim();
                if !todo.is_empty() {
                    self.state.todos.push(todo.to_string());
                    true
                } else {
                    false
                }
            }
            Message::DeleteAll => {
                info!("Receive DeleteAll");
                self.state.todos.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| {
            info!("Send DeleteAll");
            Message::DeleteAll
        });

        html! {
            <div>
                <h1>{ "Todos" }</h1>
                <DeleteAllTodosButton {onclick} value={"Delete All Todos"}/>
                { self.view_input(ctx.link()) }
                <ul>
                    { for self.state.todos.iter().map(|todo| html! {
                        <li>{ todo }</li>
                    }) }
                </ul>
            </div>
        }
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                info!("Send Add({})", value);
                Some(Message::Add(value))
            } else {
                None
            }
        });
        html! {
            <input
                placeholder="What needs to be done?"
                {onkeypress}
            />
        }
    }
}

struct DeleteAllTodosButton;

#[derive(Properties, PartialEq)]
struct DeleteAllTodosButtonProps {
    value: String,
    onclick: Callback<MouseEvent>
}

impl Component for DeleteAllTodosButton {
    type Message = Message;
    type Properties = DeleteAllTodosButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick: Callback<MouseEvent> = ctx.props().onclick.clone();
        html! {
            <button {onclick}>{ctx.props().value.clone()}</button>
        }
    }
}
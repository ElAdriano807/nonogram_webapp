mod todo_list;

use yew::{html, Component, Context, Html};
use crate::todo_list::TodoList;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<TodoList>::new().render();
}

struct App;
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <TodoList/>
        }
    }
}
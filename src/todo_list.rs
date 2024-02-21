use log::info;
use web_sys::{HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent};
use yew::{Callback, Component, Context, function_component, Html, html, Properties, TargetCast};
use Message::{Add, Search, DeleteAll, ToggleComplete};
use crate::todo_list::Message::{Delete, Edit};

#[derive(Clone, PartialEq)]
struct Todo {
    description: String,
    completed: bool,
}

pub enum Message {
    Add(String),
    Search(String),
    ToggleComplete(usize),
    Edit(usize, String),
    Delete(usize),
    DeleteAll,
}

pub struct TodoList {
    todos: Vec<Todo>,
    filter: String,
}
impl Component for TodoList {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            todos: vec![],
            filter: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Add(todo) => {
                info!("Receive Add({})", todo);
                let todo = todo;
                if !todo.is_empty() {
                    self.todos.push(Todo {
                        description: todo,
                        completed: false,
                    });
                    true
                } else {
                    false
                }
            }
            Search(filter) => {
                info!("Receive Filter({})", filter);
                self.filter = filter;
                true
            }
            ToggleComplete(index) => {
                info!("Receive ToggleComplete({})", index);
                self.todos[index].completed ^= true;
                true
            }
            Edit(index, todo) => {
                info!("Receive Edit({}, {})", index, todo);
                self.todos[index].description = todo;
                true
            }
            Delete(index) => {
                info!("Receive Delete({})", index);
                self.todos.remove(index);
                true
            }
            DeleteAll => {
                info!("Receive DeleteAll");
                self.todos.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Todos" }</h1>
                <FilterInput filter_oninput={Self::filter_oninput(ctx)}/>
                <TodoInput add_onkeypress={Self::add_onkeypress(ctx)}/>
                <ul>
                    { for self.todos.iter().filter(|todo| todo.description.matches(&self.filter).count() > 0).enumerate().map(|(index, todo)| html! {
                        <TodoListItem todo={todo.clone()} toggle_complete_onclick={Self::toggle_complete_onclick(ctx, index)} delete_onclick={Self::delete_onclick(ctx, index)} edit_onkeypress={Self::edit_onkeypress(ctx, index)}/>
                    }) }
                </ul>
                <DeleteAllTodosButton delete_all_onclick={Self::delete_all_onclick(ctx)}/>
            </div>
        }
    }
}
impl TodoList {
    fn add_onkeypress(ctx: &Context<Self>) -> Callback<KeyboardEvent> {
        ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                info!("Send Add({})", value);
                Some(Add(value))
            } else {
                None
            }
        })
    }
    fn filter_oninput(ctx: &Context<Self>) -> Callback<InputEvent> {
        ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            info!("Send Search({})", value);
            Search(value)
        })
    }
    fn edit_onkeypress(ctx: &Context<Self>, index: usize) -> Callback<KeyboardEvent> {
        ctx.link().batch_callback(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                info!("Send Edit({}, {})", index, value);
                Some(Edit(index, value))
            } else {
                None
            }
        })
    }
    fn delete_onclick(ctx: &Context<Self>, index: usize) -> Callback<MouseEvent> {
        ctx.link().callback(move |_| {
            info!("Send Delete({})", index);
            Delete(index)
        })
    }
    fn delete_all_onclick(ctx: &Context<Self>) -> Callback<MouseEvent> {
        ctx.link().callback(|_| {
            info!("Send DeleteAll");
            DeleteAll
        })
    }
    fn toggle_complete_onclick(ctx: &Context<Self>, index: usize) -> Callback<MouseEvent> {
        ctx.link().callback(move |_| {
            info!("Send ToggleComplete({})", index);
            ToggleComplete(index)
        })
    }
}

enum TodoListItemMessage {
    Edit
}
struct TodoListItem {
    edit_mode: bool,
}
#[derive(Properties, PartialEq)]
struct TodoListItemProps {
    todo: Todo,
    toggle_complete_onclick: Callback<MouseEvent>,
    delete_onclick: Callback<MouseEvent>,
    edit_onkeypress: Callback<KeyboardEvent>,
}
impl Component for TodoListItem {
    type Message = TodoListItemMessage;
    type Properties = TodoListItemProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            edit_mode: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TodoListItemMessage::Edit => {
                info!("Receive Edit");
                self.edit_mode ^= true;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let edit_onclick: Callback<MouseEvent> = ctx.link().callback(|_| {
            info!("Send Edit");
            Self::Message::Edit
        });
        let callback = ctx.props().edit_onkeypress.clone();
        let edit_onkeypress: Callback<KeyboardEvent> = ctx.link().batch_callback(move |e: KeyboardEvent| {
            callback.emit(e.clone());
            if e.key() == "Enter" {
                info!("Send Edit");
                Some(Self::Message::Edit)
            } else {
                None
            }
        });
        html! {
            <li>
                <input type="checkbox" checked={ctx.props().todo.completed} onclick={ctx.props().toggle_complete_onclick.clone()}/>
                { if self.edit_mode {
                      html!{
                          <input value={ctx.props().todo.description.clone()} onkeypress={edit_onkeypress} placeholder="What needs to be done?"/>
                      }
                  } else {
                      html!{
                          ctx.props().todo.description.clone()
                      }
                  }
                }
                <button onclick={ctx.props().delete_onclick.clone()}>{"Delete"}</button>
                <button onclick={edit_onclick}>{if self.edit_mode {"Cancel"} else {"Edit"}}</button>
            </li>
        }
    }
}

struct TodoInput;
#[derive(Properties, PartialEq)]
struct TodoInputProps {
    add_onkeypress: Callback<KeyboardEvent>,
}
impl Component for TodoInput {
    type Message = ();
    type Properties = TodoInputProps;
    fn create(_ctx: &Context<Self>) -> Self { Self }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <input onkeypress={ctx.props().add_onkeypress.clone()} placeholder="What needs to be done?"/>
        }
    }
}

struct DeleteAllTodosButton;
#[derive(Properties, PartialEq)]
struct DeleteAllTodosButtonProps {
    delete_all_onclick: Callback<MouseEvent>
}
impl Component for DeleteAllTodosButton {
    type Message = ();
    type Properties = DeleteAllTodosButtonProps;
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button onclick={ctx.props().delete_all_onclick.clone()}>
                {"Delete All Todos"}
            </button>
        }
    }
}

struct FilterInput;
#[derive(Properties, PartialEq)]
struct FilterInputProps {
    filter_oninput: Callback<InputEvent>,
}
impl Component for FilterInput {
    type Message = ();
    type Properties = FilterInputProps;
    fn create(_ctx: &Context<Self>) -> Self { Self }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input type="text" oninput={ctx.props().filter_oninput.clone()} placeholder="Filter"/>
        }
    }
}
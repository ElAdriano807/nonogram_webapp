use log::info;
use stylist::{css, StyleSource};
use web_sys::{HtmlInputElement, InputEvent, KeyboardEvent, MouseEvent};
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};
use TodoListMessage::{Add, Delete, DeleteAll, Edit, Search, ToggleComplete};

#[derive(Clone, PartialEq)]
struct Todo {
    description: String,
    completed: bool,
}

// region TodoList
pub enum TodoListMessage {
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
    type Message = TodoListMessage;
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
                <DeleteAllTodosButton delete_all_onclick={Self::delete_all_onclick(ctx)}/>
                <ul>
                    { for self.todos.iter().filter(|todo| todo.description.matches(&self.filter).count() > 0).enumerate().map(|(index, todo)| html! {
                        <TodoListItem todo={todo.clone()} toggle_complete_onclick={Self::toggle_complete_onclick(ctx, index)} delete_onclick={Self::delete_onclick(ctx, index)} edit_onkeypress={Self::edit_onkeypress(ctx, index)}/>
                    }) }
                </ul>
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
// endregion

// region TodoListItem
enum TodoListItemMessage {
    Edit(bool),
}
#[derive(PartialEq, Properties)]
pub struct TodoListItemProps {
    todo: Todo,
    toggle_complete_onclick: Callback<MouseEvent>,
    delete_onclick: Callback<MouseEvent>,
    edit_onkeypress: Callback<KeyboardEvent>,
}
struct TodoListItem {
    edit_mode: bool,
}
impl Component for TodoListItem {
    type Message = TodoListItemMessage;
    type Properties = TodoListItemProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self { edit_mode: false }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TodoListItemMessage::Edit(edit) => {
                info!("Receive Edit({})", edit);
                self.edit_mode = edit;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li>
                <input type="checkbox" checked={ctx.props().todo.completed} onclick={ctx.props().toggle_complete_onclick.clone()}/>
                { if self.edit_mode {
                      html!{
                          <input value={ctx.props().todo.description.clone()} onkeypress={Self::edit_onkeypress(ctx)} placeholder="What needs to be done?"/>
                      }
                  } else {
                      html!{
                          ctx.props().todo.description.clone()
                      }
                  }
                }
                <button onclick={Self::delete_onkeypress(ctx)} tabindex="0">{"Delete"}</button>
                <button onclick={Self::edit_onclick(ctx, self.edit_mode)} tabindex="0">{if self.edit_mode {"Cancel"} else {"Edit"}}</button>
            </li>
        }
    }
}
impl TodoListItem {
    fn edit_onclick(ctx: &Context<Self>, edit_mode: bool) -> Callback<MouseEvent> {
        ctx.link().callback(move |_| {
            info!("Send Edit({})", !edit_mode);
            TodoListItemMessage::Edit(!edit_mode)
        })
    }
    fn edit_onkeypress(ctx: &Context<Self>) -> Callback<KeyboardEvent> {
        let callback = ctx.props().edit_onkeypress.clone();
        ctx.link().batch_callback(move |e: KeyboardEvent| {
            callback.emit(e.clone());
            if e.key() == "Enter" {
                info!("Send Edit({})", false);
                Some(TodoListItemMessage::Edit(false))
            } else {
                None
            }
        })
    }
    fn delete_onkeypress(ctx: &Context<Self>) -> Callback<MouseEvent> {
        let callback = ctx.props().delete_onclick.clone();
        ctx.link().callback(move |e: MouseEvent| {
            callback.emit(e.clone());
            info!("Send Edit");
            TodoListItemMessage::Edit(false)
        })
    }
}
// endregion

// region TodoInput
enum TodoInputMessage {}
#[derive(PartialEq, Properties)]
pub struct TodoInputProps {
    add_onkeypress: Callback<KeyboardEvent>,
}
struct TodoInput;
impl Component for TodoInput {
    type Message = TodoInputMessage;
    type Properties = TodoInputProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                class={Self::style()}
                onkeypress={ctx.props().add_onkeypress.clone()}
                placeholder="What needs to be done?"
            />
        }
    }
}
impl TodoInput {
    fn style() -> StyleSource {
        css!(
            r#"
            width: 40%;
            height: 40px;
            padding: 10px 20px;
            margin: 20px 20px 20px 0px;
            border: 1px solid rgba(0, 0, 0, 0.25);
            border-left-width: 0.5px;
            background-color: rgba(0, 0, 0, 0.05);
            border-radius: 0px 20px 20px 0px;
            box-sizing: border-box;
            transition: background-color 0.3s ease;
            &:hover {
                background-color: rgba(0, 0, 0, 0.10);
            }
            &:focus {
                background-color: rgba(0, 0, 0, 0.10);
                outline: none;
                box-shadow: 0 0 3px rgba(0, 0, 0, 0.5);
            }
        "#
        )
    }
}
// endregion

// region DeleteAllTodosButton
enum DeleteAllTodosButtonMessage {}
#[derive(PartialEq, Properties)]
pub struct DeleteAllTodosButtonProps {
    delete_all_onclick: Callback<MouseEvent>,
}
struct DeleteAllTodosButton;
impl Component for DeleteAllTodosButton {
    type Message = DeleteAllTodosButtonMessage;
    type Properties = DeleteAllTodosButtonProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button class={Self::style()} onclick={ctx.props().delete_all_onclick.clone()} tabindex="0">
                {"Delete All Todos"}
            </button>
        }
    }
}
impl DeleteAllTodosButton {
    fn style() -> StyleSource {
        css!(
            r#"
            width: 10%;
            height: 40px;
            padding: 10px 20px;
            margin: 20px 20px;
            cursor: pointer;
            border: 1px solid rgba(0, 0, 0, 0.25);
            color: rgba(0, 0, 0, 0.75);
            background-color: rgba(0, 0, 0, 0.05);
            border-radius: 20px;
            transition: background-color 0.3s ease, color 0.3s ease, border-color 0.3s ease;
            &:hover {
                background-color: rgba(0, 0, 0, 0.10);
            }
            &:active {
                background-color: rgba(0, 0, 0, 0.15);
            }
            &:focus {
                background-color: rgba(0, 0, 0, 0.10);
                outline: none;
                box-shadow: 0 0 3px rgba(0, 0, 0, 0.5);
            }
        "#
        )
    }
}
// endregion

// region FilterInput
enum FilterInputMessage {}
#[derive(PartialEq, Properties)]
pub struct FilterInputProps {
    filter_oninput: Callback<InputEvent>,
}
struct FilterInput;
impl Component for FilterInput {
    type Message = FilterInputMessage;
    type Properties = FilterInputProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                class={Self::style()}
                type="text"
                oninput={ctx.props().filter_oninput.clone()}
                placeholder="Filter"
            />
        }
    }
}
impl FilterInput {
    fn style() -> StyleSource {
        css!(
            r#"
            width: 40%;
            height: 40px;
            padding: 10px 20px;
            margin: 20px 0px 20px 20px;
            border: 1px solid rgba(0, 0, 0, 0.25);
            border-right-width: 0.5px;
            background-color: rgba(0, 0, 0, 0.05);
            border-radius: 20px 0px 0px 20px;
            box-sizing: border-box;
            transition: background-color 0.3s ease;
            &:hover {
                background-color: rgba(0, 0, 0, 0.10);
            }
            &:focus {
                background-color: rgba(0, 0, 0, 0.10);
                outline: none;
                box-shadow: 0 0 3px rgba(0, 0, 0, 0.5);
            }
        "#
        )
    }
}
// endregion

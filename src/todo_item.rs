use yew::prelude::*;

pub struct TodoModel {
    completed: bool,
    text: &'static str,
    on_complete: Option<Callback<()>>,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    completed: bool,
    text: &'static str,
    on_complete: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            completed: false,
            text: "",
            on_complete: None,
        }
    }
}

pub enum Msg {
    Complete(bool),
}

impl Component for TodoModel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TodoModel {
            completed: props.completed,
            text: props.text,
            on_complete: props.on_complete,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match (msg, &self.on_complete) {
            (Msg::Complete(value), Some(callback)) => {
                callback.emit(());
            }
            (Msg::Complete(_), None) => {}
        }
        false
    }
}

impl Renderable<TodoModel> for TodoModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <p>{ &self.text }</p>
                <button onclick=|_| Msg::Complete(!self.completed),>{ "Complete" }</button>
            </div>
        }
    }
}

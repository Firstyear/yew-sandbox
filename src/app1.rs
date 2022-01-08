use gloo::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::manager::Route;

pub struct App1 {}

#[derive(Debug)]
pub enum AppMsg {
    ChangeState,
}

#[function_component(MyComponent)]
pub fn my_component() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::App2));

    html! {
        <>
            <button {onclick}>{" Next "}</button>
        </>
    }
}

impl Component for App1 {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App1 {}
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ChangeState => {
                false
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        console::log!("rendered");
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        console::log!("view");
        html! {
            <main>
                <h1>{ "Proceed" }</h1>
                <MyComponent />
            </main>
        }
    }
}

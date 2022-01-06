use gloo::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::manager::Route;

pub struct App2 {}

#[derive(Debug)]
pub enum AppMsg {
    ChangeState,
}

impl Component for App2 {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App2 {}
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ChangeState => {
                ctx.link()
                    .navigator()
                    .expect_throw("failed to read history")
                    .push(Route::App1);
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
            <body>
            <main>
              <form
                onsubmit={ ctx.link().callback(|_| {
                    AppMsg::ChangeState
                } ) }
                action="javascript:void(0);"
              >
                <h1>{ "Back" }</h1>
                <button type="submit">{ "Back" }</button>
              </form>
            </main>
            </body>
        }
    }
}

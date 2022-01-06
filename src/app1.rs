use gloo::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::manager::Route;

pub struct App1 {
    redir: bool
}

#[derive(Debug)]
pub enum AppMsg {
    ChangeState,
}

impl Component for App1 {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App1 {
            redir: false
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ChangeState => {
                self.redir = true;
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        console::log!("rendered");
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        console::log!("view");
        if self.redir {
            html! { <Redirect<Route> to={ Route::App2 }/>  }
        } else {
            html! {
                <body>
                <main>
                  <form
                    onsubmit={ ctx.link().callback(|e: FocusEvent| {
                        e.prevent_default();
                        AppMsg::ChangeState
                    } ) }
                  >
                    <h1>{ "Proceed" }</h1>
                    <button type="submit">{ "Proceed" }</button>
                  </form>
                </main>
                </body>
            }
        }
    }
}

use gloo::console;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app1::App1;
use crate::app2::App2;

// router to decide on state.
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Landing,

    #[at("/app1")]
    App1,

    #[at("/app2")]
    App2,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Landing)]
fn landing() -> Html {
    // Do this to allow use_history to work because lol.
    use_history().unwrap().push(Route::App1);
    html! { <body></body> }
}

fn switch(routes: &Route) -> Html {
    console::log!("manager::switch");
    match routes {
        Route::Landing => html! { <Landing /> },
        Route::App1 => html! { <App1 /> },
        Route::App2 => html! { <App2 /> },
        Route::NotFound => {
            html! {
                <body>
                    <h1>{ "404" }</h1>
                    <Link<Route> to={ Route::Landing }>
                    { "Home" }
                    </Link<Route>>
                </body>
            }
        }
    }
}

pub struct ManagerApp {}

impl Component for ManagerApp {
    type Message = bool;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        console::log!("manager::create");
        ManagerApp { }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        console::log!("manager::change");
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        console::log!("manager::update");
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        console::log!("manager::rendered");
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
            <head>
                <meta charset="utf-8"/>
                <title>{ "Yew Sandbox" }</title>
            </head>
            <BrowserRouter>
                <Switch<Route> render={ Switch::render(switch) } />
            </BrowserRouter>
        </>
        }
    }
}

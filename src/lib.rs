use wasm_bindgen::prelude::*;

mod manager;
mod app1;
mod app2;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<manager::ManagerApp>();
    Ok(())
}

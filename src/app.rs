use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <h1>Settings</h1>
    }
}
#[component]
pub fn New() -> impl IntoView {
    view! {
        <h1>New</h1>
    }
}
#[component]
pub fn History() -> impl IntoView {
    view! {
        <h1>History</h1>
    }
}

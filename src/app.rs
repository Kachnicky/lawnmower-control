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
pub fn History() -> impl IntoView {
    view! {
        <main>
        </main>
    }
}
#[component]
pub fn New() -> impl IntoView {
    view! {
        <main>
            <button>Create POST req</button>
        </main>
    }
}
#[component]
pub fn Presets() -> impl IntoView {
    view! {
        <main>
        </main>
    }
}

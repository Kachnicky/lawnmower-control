use std::default;
use std::error::Error;
use std::{fmt::Write, io::Read};
use std::time::{Instant, Duration};
use html::{img, P};
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use svg::view;
use tokio::runtime::{Builder, Runtime};
use wasm_bindgen::prelude::*;
use reqwest::{self, Client, Response};
use std::process::Command;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[component]
pub fn History() -> impl IntoView {
    view! {
        <main id="historyMain">
            <div id="historyTitle">
                <h1>History</h1>
            </div>
            <div class="historyItem">
                <div class="usedPreset">
                    <div class="icon">                    
                        <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M20 10C20 14.4183 12 22 12 22C12 22 4 14.4183 4 10C4 5.58172 7.58172 2 12 2C16.4183 2 20 5.58172 20 10Z"  stroke-width="2"></path><path d="M12 11C12.5523 11 13 10.5523 13 10C13 9.44772 12.5523 9 12 9C11.4477 9 11 9.44772 11 10C11 10.5523 11.4477 11 12 11Z" fill="#000000"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    </div>
                    <div class="text">
                        <h2>Whiteboard</h2>
                    </div>
                </div>
                <div class="launchedTimestamp">
                    <div class="icon">
                        <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M12 6L12 12L18 12"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    </div>
                    <div class="text">
                        <div class="date"><p>2024-11-29</p></div>
                        <div class="time"><p>07:40:54</p></div>
                    </div>
                </div>
                <div class="totalLength">
                    <div class="icon">
                        <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M9 2L15 2"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 10L12 14"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 22C16.4183 22 20 18.4183 20 14C20 9.58172 16.4183 6 12 6C7.58172 6 4 9.58172 4 14C4 18.4183 7.58172 22 12 22Z"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    </div>
                    <div class="text">
                        <div class="time"><p>00:14:43</p></div>
                    </div>    
                </div>
            </div>
            <div class="historyItem">
                <div class="usedPreset">
                    <div class="icon">                    
                        <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M20 10C20 14.4183 12 22 12 22C12 22 4 14.4183 4 10C4 5.58172 7.58172 2 12 2C16.4183 2 20 5.58172 20 10Z"  stroke-width="2"></path><path d="M12 11C12.5523 11 13 10.5523 13 10C13 9.44772 12.5523 9 12 9C11.4477 9 11 9.44772 11 10C11 10.5523 11.4477 11 12 11Z" fill="#000000"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    </div>
                    <div class="text">
                        <h2>Table</h2>
                    </div>
                </div>
                <div class="launchedTimestamp">
                    <div class="icon">
                        <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M12 6L12 12L18 12"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    </div>
                    <div class="text">
                        <div class="date"><p>2024-11-29</p></div>
                        <div class="time"><p>05:47:19</p></div>
                    </div>
                </div>
                <div class="totalLength">
                    <div class="icon">
                        <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M9 2L15 2"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 10L12 14"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path><path d="M12 22C16.4183 22 20 18.4183 20 14C20 9.58172 16.4183 6 12 6C7.58172 6 4 9.58172 4 14C4 18.4183 7.58172 22 12 22Z"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    </div>
                    <div class="text">
                        <div class="time"><p>01:35:24</p></div>
                    </div>    
                </div>
            </div>
        </main>
    }
}
#[component]
pub fn New() -> impl IntoView {
    let (playing, set_playing) = create_signal(false);
    create_effect(move |_| {
        
    });
    let getPreset = use_context::<ReadSignal<String>>().unwrap();
    let (getDiscovery, setDiscovery) = create_signal(getPreset.get().is_empty());

    view! {
        <main id="addMain">
            <div>
                <h1>Current preset : {
                    move || if getPreset.get().is_empty(){
                        return "None!".to_string()
                    }
                    else{
                        getPreset.get()
                    }
                }</h1>
            </div>
            <div id="presetImage">
                {move || match getPreset.get().as_str(){
                    "Whiteboard" => return view! {
                        <img src="public/whiteboard.png"/>
                    },
                    "Table" => return view! {
                        <img src="public/table.png"/>
                    },
                    "Garden" => return view!{
                        <img src="public/garden.png"/>
                    },
                    _ => return view!{
                        <img src="public/blank.png"/>
                    },
                }
                }
            </div>
            <div>
                <h2>Current status:</h2>
                <h3>{move || if playing.get() {
                        "Running"
                    }
                    else {
                        "Paused"
                    }
                    }
                </h3>
            </div>
            <div class="discovery">{move || if getDiscovery.get(){
                "Discovery mode enabled"
            }
            else{
                ""
            }}</div>
            <div>
            <button id="play" on:click=move |_| {
                set_playing.set(!playing.get());
                postit();
            }>
            {move || if playing.get() {
                view! { <Pause /> }
            } else {
                view! { <Play /> }
            }}  
            </button>
            </div>
            <div></div>
        </main>
    }
}

fn postit(){
    leptos::logging::log!("launched postit");
    let rt = Builder::new_current_thread().build().unwrap();
    rt.block_on(async {
            reqwest::get("http://10.10.12.238:8080").await.unwrap().text().await.unwrap();
            /* let client = reqwest::Client::new();
            let res = client.post("https://kachny.requestcatcher.com/")
            .body("LAUNCH THE SCRIPT BOY")
            .send()
            .await.unwrap();
            leptos::logging::log!("ALL GOOD BROSKI {:#?}", res); */
    });
}

#[component]
pub fn Play() -> impl IntoView {
    view!{
        <svg width="24px" height="24px" stroke-width="1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M6.90588 4.53682C6.50592 4.2998 6 4.58808 6 5.05299V18.947C6 19.4119 6.50592 19.7002 6.90588 19.4632L18.629 12.5162C19.0211 12.2838 19.0211 11.7162 18.629 11.4838L6.90588 4.53682Z" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path></svg>
    }
}
#[component]
pub fn Pause() -> impl IntoView {
    view!{
        <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M6 18.4V5.6C6 5.26863 6.26863 5 6.6 5H9.4C9.73137 5 10 5.26863 10 5.6V18.4C10 18.7314 9.73137 19 9.4 19H6.6C6.26863 19 6 18.7314 6 18.4Z"  stroke-width="2"></path><path d="M14 18.4V5.6C14 5.26863 14.2686 5 14.6 5H17.4C17.7314 5 18 5.26863 18 5.6V18.4C18 18.7314 17.7314 19 17.4 19H14.6C14.2686 19 14 18.7314 14 18.4Z"  stroke-width="2"></path></svg>
    }
}


#[component]
pub fn Presets() -> impl IntoView {
    let handle_preset_selection = move |preset_name: &str| {
        let set_preset = use_context::<WriteSignal<String>>().unwrap();
        let navigate = use_navigate(); 
        set_preset.set(preset_name.to_string());
        navigate("/", Default::default());
    };
    view! {
        <main id="presetsMain">
            <div id="presetTitle">
                <h1>Presets</h1>
            </div>

            <button class="preset" on:click=move |_| {
                handle_preset_selection("Garden");
            }>
                <div class="presetTop">
                    <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M20 10C20 14.4183 12 22 12 22C12 22 4 14.4183 4 10C4 5.58172 7.58172 2 12 2C16.4183 2 20 5.58172 20 10Z"  stroke-width="2"></path><path d="M12 11C12.5523 11 13 10.5523 13 10C13 9.44772 12.5523 9 12 9C11.4477 9 11 9.44772 11 10C11 10.5523 11.4477 11 12 11Z" fill="#000000"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    <h2>Garden</h2>
                </div>
                <div class="presetBottom">
                    <img src="public/garden.png"></img>
                </div>
            </button>

            <button class="preset" on:click=move |_| {
                handle_preset_selection("Table");
            }>
                <div class="presetTop">
                    <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M20 10C20 14.4183 12 22 12 22C12 22 4 14.4183 4 10C4 5.58172 7.58172 2 12 2C16.4183 2 20 5.58172 20 10Z"  stroke-width="2"></path><path d="M12 11C12.5523 11 13 10.5523 13 10C13 9.44772 12.5523 9 12 9C11.4477 9 11 9.44772 11 10C11 10.5523 11.4477 11 12 11Z" fill="#000000"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    <h2>Table</h2>
                </div>
                <div class="presetBottom">
                    <img src="public/table.png"></img>
                </div>
            </button>
            <button class="preset" on:click=move |_| {
                handle_preset_selection("Whiteboard");
            }>
                <div class="presetTop">
                    <svg width="24px" height="24px" stroke-width="2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000"><path d="M20 10C20 14.4183 12 22 12 22C12 22 4 14.4183 4 10C4 5.58172 7.58172 2 12 2C16.4183 2 20 5.58172 20 10Z"  stroke-width="2"></path><path d="M12 11C12.5523 11 13 10.5523 13 10C13 9.44772 12.5523 9 12 9C11.4477 9 11 9.44772 11 10C11 10.5523 11.4477 11 12 11Z" fill="#000000"  stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path></svg>
                    <h2>Whiteboard</h2>
                </div>
                <div class="presetBottom">
                    <img src="public/whiteboard.png"></img>
                </div>
            </button>
        </main>
    }
}

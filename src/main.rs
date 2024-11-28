mod app;

use app::*;
use leptos::*;
use leptos_router::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Router>

            <main>
                <Routes>
                    <Route path="/presets" view=Presets/>
                    <Route path="/" view=New/>
                    <Route path="/history" view=History/>
                </Routes>
            </main>
            <Nav/>
        </Router>
        }
    })
}
#[component]
pub fn Nav() -> impl IntoView {
    view!{
        <nav>
            <a href="/presets"><svg stroke-width="1" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M8 6L20 6" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path><path d="M4 6.01L4.01 5.99889"  stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path><path d="M4 12.01L4.01 11.9989"  stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path><path d="M4 18.01L4.01 17.9989"  stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path><path d="M8 12L20 12"  stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path><path d="M8 18L20 18"  stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path></svg></a>
            <a href="/"><svg stroke-width="1" viewBox="0 0 24 24"   xmlns="http://www.w3.org/2000/svg"><path d="M6 12H12M18 12H12M12 12V6M12 12V18" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path></svg></a>
            <a href="/history"><svg viewBox="0 0 24 24" fill="none" stroke-width="1" xmlns="http://www.w3.org/2000/svg"><path d="M12 6L12 12L18 12" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path><path d="M21.8883 10.5C21.1645 5.68874 17.013 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22C16.1006 22 19.6248 19.5318 21.1679 16" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path><path d="M17 16H21.4C21.7314 16 22 16.2686 22 16.6V21" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"></path></svg></a>
        </nav>
    }
}

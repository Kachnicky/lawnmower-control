mod app;

use app::*;
use leptos::*;
use leptos_router::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Router>
            <Nav/>
            <main>
                <Routes>
                    <Route path="/settings" view=Settings/>
                    <Route path="/" view=New/>
                    <Route path="/history" view=History/>
                </Routes>
            </main>
        </Router>
        }
    })
}
#[component]
pub fn Nav() -> impl IntoView {
    view!{
        <nav>
            <a href="/settings">Settings</a>
            <a href="/">New</a>
            <a href="/history">history</a>
        </nav>
    }
}

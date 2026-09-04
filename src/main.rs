use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Test"</h1>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
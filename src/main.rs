use leptos::*;

#[component]
fn App(_cx: Scope) -> Element {
    view! { _cx,
        <p>"Hello, world!"</p>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}

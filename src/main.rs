use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Hello, world!"</p>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}

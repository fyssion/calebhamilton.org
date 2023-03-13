use leptos::*;
use leptos_meta::*;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="fyssion's bio"/>
        <h1>"About me"</h1>
        <p>"I'm a self-taught programmer."</p>
    }
}

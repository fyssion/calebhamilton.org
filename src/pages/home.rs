use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="fyssion's zone"/>
        <h1>"Welcome"</h1>
        <p>"I'm Fyssion. I enjoy writing software."</p>

        <p>"You can find some of my work on my "<a href="https://github.com/Fyssion">"GitHub"</a>"."</p>
    }
}

use leptos::*;
use crate::utils::page_meta::PageMeta;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <PageMeta title="fyssion's zone" description="Fyssion's personal website" />

        <h1>"Welcome"</h1>
        <p>"I'm Fyssion. I enjoy writing software."</p>

        <p>"You can find some of my work on my "<a href="https://github.com/Fyssion">"GitHub"</a>"."</p>
    }
}

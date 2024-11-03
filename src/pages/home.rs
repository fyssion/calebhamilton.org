use crate::utils::page_meta::PageMeta;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <PageMeta title="Caleb Hamilton" description="Caleb's personal website" />

        <h1>"Welcome"</h1>
        <p>"I'm Caleb. You might also see me as Fyssion. I enjoy writing software."</p>

        <p>"You can find some of my work on my "<a href="https://github.com/Fyssion">"GitHub"</a>"."</p>
    }
}

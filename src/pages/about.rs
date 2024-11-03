use crate::utils::page_meta::PageMeta;
use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <PageMeta title="Caleb's Bio" description="About Caleb" />

        <h1>"About me"</h1>
        <p>"I'm a self-taught programmer."</p>
    }
}

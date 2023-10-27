use leptos::*;
use crate::utils::page_meta::PageMeta;

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <PageMeta title="fyssion's bio" description="About Fyssion" />

        <h1>"About me"</h1>
        <p>"I'm a self-taught programmer."</p>
    }
}

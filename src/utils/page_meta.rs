use leptos::{prelude::*, text_prop::TextProp};
use leptos_meta::*;

#[component]
pub fn PageMeta(
    #[prop(into)] title: TextProp,
    #[prop(into)] description: TextProp,
) -> impl IntoView {
    view! {
        <Title text={title.clone()}/>
        <Meta name="description" content={description.clone()} />
        <Meta name="og:title" content={title} />
        <Meta name="og:description" content={description} />
    }
}

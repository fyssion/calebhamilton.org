use leptos::*;
use leptos_meta::*;

#[component]
pub fn PageMeta(
    #[prop(into)] title: TextProp,
    #[prop(into)] description: TextProp,
    // idk wtf i'm doing
    // #[prop(default = Box::new(|s| Fragment::new(Vec::new()) ) )] children: Children,
) -> impl IntoView {
    view! {
        <Title text={title.clone()}/>
        <Meta name="description" content={description.clone()} />
        <Meta name="og:title" content={title} />
        <Meta name="og:description" content={description} />

        // {children()}
    }
}

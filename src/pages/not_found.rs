use leptos::*;
use leptos_meta::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="Not Found | Caleb Hamilton"/>
        <div class="container">
            <div class="content">
                <h1 style="text-align: center">"Aw shucks"</h1>
                <p style="text-align: center">"This is a 404 page."</p>
            </div>
        </div>
    }
}

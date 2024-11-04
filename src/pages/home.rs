use crate::utils::page_meta::PageMeta;
use leptos::*;
use leptos_router::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <PageMeta title="Caleb Hamilton" description="Caleb's personal website" />

        <h1>"Welcome"</h1>

        <p>"I'm Caleb. You might also see me as Fyssion."</p>
        <p>
            "I'm currently a student at the University of Wisconsin - Madison "
            "studying computer science, but my passion for creating software long precedes "
            "my university studies. Starting with Python in middle school, I've enjoyed teaching myself "
            "how to use a wide range of programming languages and technologies throughout the years."
        </p>
        <p>
            "You can find some of my favorite projects on my "
            <A href="/projects">"projects page"</A>
            ", see what I've been thinking about on "
            <A href="/blog">"my blog"</A>
            ", or browse all my public work on my "
            <a href="https://github.com/Fyssion">"GitHub"</a>"."
        </p>
    }
}

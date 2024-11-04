use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{
    blog::Blog,
    home::Home,
    not_found::NotFound,
    projects::Projects,
    post::BlogPost,
};

static SITE_TITLE: &'static str = "Caleb Hamilton";

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" />

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/calebhamilton_org.css"/>

        <Title text=SITE_TITLE/>

        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <Link rel="manifest" href="/site.webmanifest" />
        <Meta name="msapplication-TileColor" content="#272838" />
        <Meta name="theme-color" content="#272838" />

        <Script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js" />
        <Link rel="preconnect" href="https://rsms.me/" />
        <Link rel="stylesheet" href="https://rsms.me/inter/inter.css" />

        //  TODO: add real error handling
        <Router fallback=move || {view!{ <NotFound />}.into_view()}>
        <div class="app">
                <header>
                    <nav>
                        <div class="logo">
                            <A exact=true href="/">"Caleb Hamilton"</A>
                        </div>
                        <ul>
                            <li>
                                <A href="blog">"blog"</A>
                            </li>
                            <li>
                                <A href="projects">"projects"</A>
                            </li>
                        </ul>
                    </nav>
                </header>

                <main>
                    <Routes>
                        <Route path="" view=|| view! { <Home/> }/>
                        <Route path="projects" view=|| view! { <Projects/> }/>
                        <Route path="blog" view=|| view! { <Blog/> }/>
                        <Route
                            path="/blog/:id"
                            view=|| view! { <BlogPost/> }
                            ssr=leptos_router::SsrMode::Async
                        />
                    </Routes>
                </main>
                <footer>
                    <ul>
                        <li>
                            <a href="https://github.com/Fyssion" target="_blank">
                                <img src="/github.svg" />"Fyssion"
                            </a>
                        </li>
                        <li>
                            <a href="https://www.linkedin.com/in/calebthamilton/" target="_blank">
                            <img src="/linkedin.svg" />"calebthamilton"
                        </a>
                        </li>
                    </ul>
                </footer>
            </div>
        </Router>
    }
}

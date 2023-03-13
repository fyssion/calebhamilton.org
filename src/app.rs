use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{
    blog::{Blog, BlogProps},
    home::{Home, HomeProps},
    not_found::{NotFound, NotFoundProps},
    // about::{AboutProps, About},
    post::{BlogPost, BlogPostProps},
};

static SITE_TITLE: &'static str = "fyssion's zone";

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fyssion_zone.css"/>

        <Title text=SITE_TITLE/>

        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <Link rel="manifest" href="/site.webmanifest" />
        <Meta name="msapplication-TileColor" content="#EB9486" />
        <Meta name="theme-color" content="#EB9486" />

        //  TODO: add real error handling
        <Router fallback=move |cx| {view!{ cx, <NotFound />}.into_view(cx)}>
        <div class="app">
                <header>
                    <nav>
                        <div class="logo">
                            <A exact=true href="/">"fyssion.zone"</A>
                            // <A href="about">"about"</A>  // not yet!!
                        </div>
                        <ul>
                            <li>
                                <A href="blog">"blog"</A>
                            </li>
                            <li>
                                <a href="https://github.com/Fyssion">"github"</a>
                            </li>
                        </ul>
                    </nav>
                </header>

                <main>
                    <Routes>
                        <Route path="" view=|cx| view! { cx, <Home/> }/>
                        // <Route path="about" view=|cx| view! { cx, <About/> }/>
                        <Route path="blog" view=|cx| view! { cx, <Blog/> }/>
                        <Route
                            path="/blog/post/:id"
                            view=|cx| view! { cx, <BlogPost/> }
                            ssr=leptos_router::SsrMode::Async
                        />
                    </Routes>
                </main>
                // <footer>
                //     <p>"Made with Rust using Leptos!"</p>
                // </footer>
            </div>
        </Router>
    }
}

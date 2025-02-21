use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::*,
    path,
};

use crate::pages::{
    blog::Blog, home::Home, not_found::NotFound, post::BlogPost, projects::Projects,
};

static SITE_TITLE: &'static str = "Caleb Hamilton";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" id="leptos" href="/pkg/calebhamilton_org.css"/>
                <title>{SITE_TITLE}</title>

                <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
                <link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
                <link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
                <link rel="manifest" href="/site.webmanifest" />
                <meta name="msapplication-TileColor" content="#272838" />
                <meta name="theme-color" content="#272838" />

                <Script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js" />
                <link rel="preconnect" href="https://rsms.me/" />
                <link rel="stylesheet" href="https://rsms.me/inter/inter.css" />

                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        //  TODO: add real error handling
        <Router>
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
                    <Routes fallback=move || {view!{ <NotFound />}.into_view()}>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/projects") view=Projects />
                        <Route path=path!("/blog") view=Blog />
                        <Route
                            path=path!("/blog/:id")
                            view=BlogPost
                            ssr=leptos_router::SsrMode::Async
                        />
                    </Routes>
                </main>
                <footer>
                    <ul>
                        <li>
                            <a rel="external" href="https://github.com/Fyssion" target="_blank">
                                <img src="/github.svg" />"Fyssion"
                            </a>
                        </li>
                        <li>
                            <a rel="external" href="https://www.linkedin.com/in/calebthamilton/" target="_blank">
                            <img src="/linkedin.svg" />"calebthamilton"
                        </a>
                        </li>
                    </ul>
                </footer>
            </div>
        </Router>
    }
}

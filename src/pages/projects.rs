use crate::utils::page_meta::PageMeta;
use leptos::*;
use leptos_router::A;

#[component]
pub fn Project(
    children: Children,
    name: &'static str,
    timeframe: &'static str,
    // #[prop(optional)]
    // url: &'static str,
    #[prop(optional)] github: &'static str,
    #[prop(optional)] image: &'static str,
) -> impl IntoView {
    let cover_image = move || {
        (!image.is_empty()).then(|| {
            view! {
                <img class="project-cover" src={image} loading="lazy" />
            }
        })
    };

    let github_icon = move || {
        (!github.is_empty()).then(|| {
            view! {
                <a href={format!("https://github.com/{}", github)} target="_blank">
                    <img src="/github.svg" />
                    <span>{github}</span>
                </a>
            }
        })
    };

    view! {
        <li>
            <div class="project-cover-wrapper">
                {cover_image}
            </div>
            <div class="project-meta">
                <div class="project-header">
                    <h2>{name}</h2>
                    {github_icon}
                </div>
                <h3>{timeframe}</h3>
                {children()}
            </div>
        </li>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <PageMeta title="Projects - Caleb Hamilton" description="Caleb's projects" />

        <h1>"Projects"</h1>
        <p>"These are some of my favorite projects over the years."</p>

        <ul class="projects-list">
            <Project
                name="This site"
                timeframe="2023 \u{2013} present"
                github="Fyssion/calebhamilton.org"
                image="/blog.webp"
            >
                <p>
                    "This website was created as a markdown blogging engine in Rust. "
                    "It includes a client compiled to WebAssembly and an API server. "
                    "To build it, I used fine-grained reactivity and SSR to create a fast, robust full-stack web app. "
                </p>
                <p>
                    "I also used this project to learn how to use Docker. "
                    "I wrote a Dockerfile, which is automatically built with GitHub Actions whenever I push to the remote."
                </p>
                <p>
                    "See " <A href="/blog/making-a-blog">"my blog post"</A> " for more info on how I created it."
                </p>
            </Project>
            <Project
                name="Zupplin"
                timeframe="2021 \u{2013} 2023"
                github="Fyssion/zupplin"
                image="/zupplin.webp"
            >
                <p>
                    "Zupplin was a toy chat server I made for fun. "
                    "It was a fully-featured chat platform complete with profiles, group chats, friends, and more. "
                    "I created a lightweight front-end with React, Redux, JavaScript/TypeScript, and HTML/CSS to interact with "
                    "the back-end infrastructure I built and deployed using Python, Tornado, Docker, PostgreSQL, and other technologies."
                </p>
                <p>
                    "I also used this project to learn how to automate CI/CD using GitHub Actions. "
                    "I wrote workflows to triage issues and PRs, lint the codebase, and build the front-end."
                </p>
            </Project>
            <Project
                name="Clam"
                timeframe="2019 \u{2013} 2022"
                github="Fyssion/Clam"
                image="/clam.webp"
            >
                <p>
                    "Clam was a Discord bot written in Python that interacted with the Discord API. "
                    "It was originally created as a fun, simple testing bot "
                    "that I used to experiment with new programming concepts and learn different technologies. "
                    "Clam ended up being used by my friends for music, a starboard, moderation, and more. "
                </p>
                <p>
                    "While writing Clam, I learned how to use PostgreSQL and asyncpg. "
                    "I also learned web scraping and how to interact with a variety of REST APIs to provide extra functionality."
                </p>
            </Project>
        </ul>
    }
}

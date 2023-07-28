use leptos::*;

use crate::utils::page_meta::{PageMeta};
use crate::utils::post::get_post_metadata;

#[component]
fn SkeletonPost(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="skeleton-card">
            <div class="skeleton skeleton-header"></div>
            <div class="skeleton skeleton-text skeleton-details"></div>
            <div class="skeleton skeleton-text"></div>
            <div class="skeleton skeleton-text"></div>
        </div>
    }
}

#[component]
pub fn Blog(cx: Scope) -> impl IntoView {
    let posts = create_resource(cx, || (), |_| async { get_post_metadata().await });
    let posts_view = move || {
        posts.with(cx, |posts| {
            posts.clone().map(|posts| {
                posts
                    .iter()
                    .filter(|p| p.hidden == false)
                    .map(|post| {
                        view! { cx,
                            <li>
                                <a href=format!("/blog/post/{}", post.id)>
                                    <h2>{&post.title}</h2>
                                </a>
                                <h3>{
                                    &post.created_at.format("%b %e, %Y").to_string()
                                }</h3>
                                <p>{&post.description}</p>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
            })
        })
    };

    view! { cx,
        <PageMeta title="fyssion's blog" description="Thoughts and reflections without thinking or reflecting" />

        <h1>"Blog"</h1>
        <p>"An RSS feed for this blog is available "<a target="_blank" href="/blog/feed.rss">"here"</a>"."</p>
        <Suspense fallback=move || view! { cx,
            <SkeletonPost />
            <SkeletonPost />
        }>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx,
                    <div class="error">
                        <h2>"Aw shucks"</h2>
                        <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, error)| view! { cx, <li>{error.to_string()} </li> })
                            .collect::<Vec<_>>()
                        }
                        </ul>
                    </div>
                }
            }>
                <ul class="posts-list">{posts_view}</ul>
            </ErrorBoundary>
        </Suspense>
    }
}

use leptos::prelude::*;
use leptos_meta::Link;

use crate::utils::page_meta::PageMeta;
use crate::utils::post::get_post_metadata;
use crate::utils::post_meta::PostMeta;

static SKELETON_POSTS: u8 = 1;

#[component]
fn SkeletonPost() -> impl IntoView {
    view! {
        <div class="skeleton-card">
            <div class="skeleton skeleton-header"></div>
            <div class="skeleton skeleton-text skeleton-details"></div>
            <div class="skeleton skeleton-text"></div>
            <div class="skeleton skeleton-text"></div>
        </div>
    }
}

#[component]
pub fn Blog() -> impl IntoView {
    let posts = Resource::new(|| (), |_| async { get_post_metadata().await });
    let posts_view = move || {
        posts.map(|posts| {
            posts.clone().map(|posts| {
                posts
                    .iter()
                    .filter(|p| p.hidden == false)
                    .map(|post| {
                        view! {
                            <li>
                                <a href=format!("/blog/{}", post.id)>
                                    <h2>{post.title.clone()}</h2>
                                </a>
                                <PostMeta metadata={post.clone()} />
                                <p>{post.description.clone()}</p>
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()
            })
        })
    };

    view! {
        <PageMeta title="Caleb's Blog" description="Thoughts and reflections without thinking or reflecting" />
        <Link rel="alternate" type_="application/rss+xml" title="RSS Feed" href="/blog/feed.rss" />

        <h1>"Blog"</h1>
        <p>"An RSS feed for this blog is available "<a target="_blank" href="/blog/feed.rss">"here"</a>"."</p>
        <Suspense fallback=move || view! {
            {(0..SKELETON_POSTS).into_iter()
                .map(|_| view! {<SkeletonPost />})
                .collect::<Vec<_>>()}
        }>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <h2>"Aw shucks"</h2>
                        <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, error)| view! { <li>{error.to_string()} </li> })
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

use leptos::logging::log;
use leptos::prelude::*;
use leptos_meta::*;
use leptos::Params;
use leptos_router::params::Params;
use leptos_router::hooks::use_params;

use crate::utils::page_meta::PageMeta;
use crate::utils::post_meta::PostMeta;
use crate::utils::{errors::PostError, post::get_post};

#[component]
fn SkeletonPost() -> impl IntoView {
    view! {
        <div class="skeleton-card-full title">
            <div class="skeleton skeleton-header title"></div>
            <div class="skeleton skeleton-text skeleton-details"></div>
            {(0..12).into_iter()
                .map(|_| view! {<div class="skeleton skeleton-text" />})
                .collect::<Vec<_>>()}
        </div>
    }
}

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct BlogPostParams {
    id: String,
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let query = use_params::<BlogPostParams>();
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.id.clone())
                .map_err(|_| PostError::InvalidTitle)
        })
    };
    let post = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_post(id.clone())
                .await
                .map(|data| data.ok_or(PostError::PostNotFound))
                .map_err(|e| {
                    log!("{:?}", e);
                    PostError::ServerError
                })
                .flatten(),
        }
    });

    let post_view = move || {
        post.map(|post| {
            post.clone().map(|post| {
                view! {
                        <article>
                        // render content
                        <h1 class="title">{post.metadata.title.clone()}</h1>
                        <PostMeta metadata={post.metadata.clone()} />
                        <section inner_html={post.content.clone()} />
                    </article>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    // <Title text={format!("{} - Caleb's Blog", post.metadata.title)}/>
                    // <Meta name="description" content=post.metadata.description/>
                    <PageMeta title={format!("{} - Caleb's Blog", post.metadata.title.clone())} description=post.metadata.description.clone() />
                    <Meta name="og:type" content="article"/>
                    <Meta name="article:published_time" content={post.metadata.created_at.to_rfc3339()}/>
                }
            })
        })
    };

    view! {
        <Suspense fallback=move || view! { <SkeletonPost /> }>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <h1>"Aw shucks"</h1>
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
                {post_view}
            </ErrorBoundary>
        </Suspense>
    }
}

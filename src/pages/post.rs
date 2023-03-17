use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::utils::{errors::PostError, post::get_post};
use crate::utils::page_meta::{PageMeta, PageMetaProps};

#[component]
fn SkeletonPost(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="skeleton-card-full title">
            <div class="skeleton skeleton-header title"></div>
            <div class="skeleton skeleton-text skeleton-details"></div>
            <div class="skeleton skeleton-text"></div>
            <div class="skeleton skeleton-text"></div>
            <div class="skeleton skeleton-text"></div>
            <div class="skeleton skeleton-text"></div>
        </div>
    }
}

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct BlogPostParams {
    id: String,
}

#[component]
pub fn BlogPost(cx: Scope) -> impl IntoView {
    let query = use_params::<BlogPostParams>(cx);
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.id.clone())
                .map_err(|_| PostError::InvalidTitle)
        })
    };
    let post = create_resource(cx, id, |id| async move {
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
        post.with(cx, |post| {
            post.clone().map(|post| {
                view! { cx,
                        <article>
                        // render content
                        <h1 class="title">{&post.metadata.title}</h1>
                        <h3>
                            {&post.metadata.created_at.format("%b %e, %Y").to_string()}
                            " • "
                            {words_count::count(&post.content).words}
                            " words"
                        </h3>
                        <section inner_html={&post.content} />
                    </article>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    // <Title text={format!("{} - fyssion's blog", post.metadata.title)}/>
                    // <Meta name="description" content=post.metadata.description/>
                    <PageMeta title={format!("{} - fyssion's blog", post.metadata.title)} description=post.metadata.description>
                        <Meta name="og:type" content="article"/>
                        <Meta name="article:published_time" content={post.metadata.created_at.to_rfc3339()}/>
                    </PageMeta>

                    // need to do this after post loads
                    <script>"hljs.highlightAll();"</script>
                }
            })
        })
    };

    view! { cx,
        <Suspense fallback=move || view! { cx, <SkeletonPost /> }>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx,
                    <div class="error">
                        <h1>"Aw shucks"</h1>
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
                {post_view}
            </ErrorBoundary>
        </Suspense>
    }
}

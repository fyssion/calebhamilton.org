use cfg_if::cfg_if;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        routing::get,
        Router,
    };
    use calebhamilton_org::app::*;
    use calebhamilton_org::pages::fallback::file_and_error_handler;
    use calebhamilton_org::pages::feed::feed;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos::logging::log;
    use tower::ServiceBuilder;
    use tower_http::trace::TraceLayer;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|| view! { <App/> });

    // init logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("debug,hyper=info,mio=info"))
    }

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .route("/blog/feed.rss", get(feed))
        .route("/blog/post/:post", get(post_redirect))
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    // HttpServer::new(move || {
    //     let leptos_options = &conf.leptos_options;
    //     let site_root = &leptos_options.site_root;

    //     App::new()
    //         .service(feed)
    //         .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
    //         .leptos_routes(
    //             leptos_options.to_owned(),
    //             routes.to_owned(),
    //             || view! { <App/> },
    //         )
    //         .service(Files::new("/", site_root))
    //     //.wrap(middleware::Compress::default())
    // })
    // .bind(&addr)?
    // .run()
    // .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

// blog posts used to be routed under /blog/post/<post>, but
// I switched it to /blog/<post>, so this handler redirects
// old post URLs
cfg_if! {
    if #[cfg(feature = "ssr")] {

        use axum::{extract::Path, response::{IntoResponse, Redirect}};

        async fn post_redirect(Path(post): Path<String>) -> impl IntoResponse {
            let uri = format!("/blog/{}", post);
            Redirect::to(&uri).into_response()
        }
    }
}

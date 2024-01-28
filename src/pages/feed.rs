use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

        use axum::{http::{StatusCode, header}, response::IntoResponse};
    use rss::{ChannelBuilder, GuidBuilder, ItemBuilder};
    use crate::utils::post::get_post_metadata;

    pub async fn feed() -> impl IntoResponse {
        let posts = get_post_metadata().await.unwrap();

        let channel = ChannelBuilder::default()
            .title("Fyssion's blog")
            .link("https://fyssion.zone/blog/")
            .description("Thoughts and reflections without thinking or reflecting")
            .language(Some("en-us".to_owned()))
            .copyright(Some("Copyright 2023-present Fyssion".to_owned()))
            .managing_editor(Some("fyssioncodes@gmail.com (Fyssion)".to_owned()))
            .webmaster(Some("fyssioncodes@gmail.com (Fyssion)".to_owned()))
            .items(
                posts.iter()
                    .filter(|p| p.hidden == false)
                    .map(|p| {
                        ItemBuilder::default()
                            .title(Some(p.title.to_owned()))
                            .link(Some(format!("https://fyssion.zone/blog/post/{}/", p.id.to_owned()).to_owned()))
                            .description(Some(p.description.to_owned()))
                            .author(Some("fyssioncodes@gmail.com (Fyssion)".to_owned()))
                            .guid(
                                GuidBuilder::default()
                                    .value(format!("https://fyssion.zone/blog/post/{}/", p.id.to_owned()).to_owned())
                                    .permalink(true)
                                    .build()
                            ) // TODO: add post content (requires some code restructuring)
                            .pub_date(Some(p.created_at.to_rfc2822()))
                            .build()
                    })
                    .collect::<Vec<_>>()
            )
            .build();

        (StatusCode::OK, [(header::CONTENT_TYPE, "text/xml; charset=utf-8")], channel.to_string())
    }
}
}

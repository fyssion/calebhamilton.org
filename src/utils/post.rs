use cfg_if::cfg_if;
use comrak::{ExtensionOptions, Options};
use lazy_static::lazy_static;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref COMRAK_OPTIONS: Options<'static> = Options {
        extension: ExtensionOptions {
            strikethrough: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            footnotes: true,
            description_lists: true,
            front_matter_delimiter: Some("---".to_owned()),
            ..Default::default()
        },
        ..Default::default()
    };
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::utils::errors::LoadingError;
    use syntect::highlighting::ThemeSet;
    use std::ops::Add;
    use comrak::{plugins::syntect::SyntectAdapterBuilder, ComrakPlugins, ComrakRenderPlugins};


    pub async fn load_posts() -> Result<Vec<PostMetadata>, LoadingError> {

        let mut posts = vec![];
        let mut entries = tokio::fs::read_dir("posts").await?;

        while let Some(entry) = entries.next_entry().await? {
            let id = entry
                .path()
                .file_stem()
                .map_or_else(String::new, |filename| {
                    let filename = filename.to_string_lossy().into_owned();
                    match filename.strip_suffix(".md") {
                        Some(filename) => filename.to_string(),
                        None => filename,
                    }
                });

            let content = tokio::fs::read_to_string(entry.path()).await?;
            posts.push(PostMetadata::build(&id, &content)?);

        }

        posts.sort_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap());
        Ok(posts)
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ParsedPostMetadata {
        pub title: String,
        pub description: String,
        #[serde(with = "chrono::serde::ts_seconds")]
        pub created_at: chrono::DateTime<chrono::Utc>,
        #[serde(default = "default_hidden")]
        pub hidden: bool,
    }

    fn default_hidden() -> bool {
        return false;
    }

    // with a little help from kaylynn's website
    // https://github.com/kaylynn234/website/blob/462d31712d5ac2bd36ce024bfceeb1622ca7d478/website/src/page.rs#L149
    fn traverse<'a>(root: &'a comrak::nodes::AstNode<'a>) -> impl Iterator<Item = &'a comrak::nodes::AstNode<'a>> {
        root.traverse().filter_map(|edge| match edge {
            comrak::arena_tree::NodeEdge::Start(node) => Some(node),
            comrak::arena_tree::NodeEdge::End(_) => None,
        })
    }

    struct PostCounter {
        pub length: usize,
        pub word_count: usize,
    }

    impl PostCounter {
        pub fn from_string(string: String) -> Self {
            PostCounter {
                length: string.chars().count(),
                word_count: words_count::count(string).words,
            }
        }

        pub fn empty() -> Self {
            PostCounter {
                length: 0,
                word_count: 0,
            }
        }
    }

    impl Add<PostCounter> for PostCounter {
        type Output = PostCounter;

        fn add(self, other: PostCounter) -> PostCounter {
            PostCounter {
                length: self.length + other.length,
                word_count: self.word_count + other.word_count,
            }
        }
    }

    impl PostMetadata {
        pub fn build(id: &String, content: &String) -> Result<Self, LoadingError> {
            let arena = comrak::Arena::new();
            let root = comrak::parse_document(&arena, content, &COMRAK_OPTIONS);

            let front_matter = traverse(root).find_map(|node| match node.data.borrow().value {
                comrak::nodes::NodeValue::FrontMatter(ref s) => Some(s.clone()),
                _ => None,
            });

            let counter = traverse(root)
                .map(|node| match node.data.borrow().value {
                    comrak::nodes::NodeValue::Text(ref s) => PostCounter::from_string(s.clone()),
                    comrak::nodes::NodeValue::Code(ref code) => PostCounter::from_string(code.literal.clone()),
                    comrak::nodes::NodeValue::CodeBlock(ref code) => PostCounter::from_string(code.literal.clone()),
                    _ => PostCounter::empty(),
                })
                .reduce(|acc, e| acc + e)
                .expect("there to be content in the post");

            let parsed_metadata: ParsedPostMetadata = toml::from_str(front_matter.unwrap_or_default().trim().trim_matches('-'))?;

            Ok(Self {
                id: id.to_owned(),
                title: parsed_metadata.title,
                description: parsed_metadata.description,
                created_at: parsed_metadata.created_at,
                hidden: parsed_metadata.hidden,
                length: counter.length,
                word_count: counter.word_count,
            })
        }
    }

}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub metadata: PostMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostMetadata {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub hidden: bool,
    pub length: usize,
    pub word_count: usize,
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Option<Post>, ServerFnError> {
    let post = match tokio::fs::read_to_string(format!("posts/{}.md", id)).await {
        Ok(p) => {
            let theme_set = ThemeSet::load_from_folder("vendor").unwrap();
            let adapter = SyntectAdapterBuilder::new()
                .theme_set(theme_set)
                .theme("Enki-Tokyo-Night")
                .build();
            let plugins = ComrakPlugins {
                render: ComrakRenderPlugins {
                    codefence_syntax_highlighter: Some(&adapter),
                    ..Default::default()
                },
                ..Default::default()
            };

            let content = comrak::markdown_to_html_with_plugins(&p, &COMRAK_OPTIONS, &plugins);
            let metadata = match PostMetadata::build(&id, &p) {
                Ok(m) => Ok(m),
                Err(e) => Err(ServerFnError::new(e.to_string())),
            }?;
            Some(Post {
                id,
                content,
                metadata,
            })
        }
        Err(_) => None,
    };
    Ok(post)
}

#[server(GetPostMetadata, "/api")]
pub async fn get_post_metadata() -> Result<Vec<PostMetadata>, ServerFnError> {
    // I know this returns hidden posts too, which isn't the best... oh well
    // I should also prolly cache this stuff so I'm not reading a bunch of files
    // every time this endpoint is called... maybe later
    // TODO ^^
    match load_posts().await {
        Ok(p) => Ok(p),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

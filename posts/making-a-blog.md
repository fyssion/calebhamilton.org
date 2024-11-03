---
title = "Making a blog"
description = "Blogging is hard."
created_at = 1679084830
---

I've been wanting to start a blog for some time now.
Thing is, I could never figure out how.
Not necessarily how to build the blog, but how to motivate myself to start and finish.
Of course, I could always throw together a simple Jekyll site, toss it on GitHub pages, and call it a day.
In fact, I've done that a few times already. Maybe a few too many!
Well, perhaps I could use a different blogging engine or even build a little bloggy thing myself?
But, hmm, that still doesn't seem interesting enough.
Lately I've been kinda tired of Python as well as the ever-changing world of JavaScript frameworks.

This is also to be my personal blog, after all, tied directly to me,
which means the pressure's on, huh? At least that's how I felt.
It's a bit silly, but so am I. So for a couple of years, nothing happened.

In November 2022, I discovered [Leptos][leptos], a Rust framework for making web apps.
It seemed pretty neat, so I thought I'd give it a go.
In the weeks prior, I had been slowly exploring the world of WebAssembly and taking a surface-level peek at browser internals.
I was also trying to find new ways to learn Rust, so writing a fully-fledged web app in the language was rather appealing.
Why not build that blog I'd been wanting to write?
Maybe I'd even put it online.

I started with the [Leptos starter template][leptos-starter] using a nifty tool called `cargo-leptos`.
Leptos makes it pretty easy to make a simple Wasm-powered web app.
It's probably most comparable to SolidJS, meaning it utilizes fine-grained reactivity for performant code
and does not use a virtual DOM.

This was all pretty exciting to me,
so I quickly whipped up a home page that looked a bit like this:

```rust
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Welcome"</h1>
        <p>"I'm Fyssion. I enjoy writing software."</p>
    }
}
```

Pretty simple, huh?
The `view!` macro resembles JSX,
which you're probably familiar with if you've used a React-like library before.

Next, with a simple main method, you can mount this "app" to the DOM.

```rust
pub fn main() {
    mount_to_body(|cx| view! { cx,  <App /> })
}
```

Congrats! You now have a reactive, Rust-powered Wasm web app ready to roll!
You could bundle this into a site with a tool like [`trunk`][trunk].

Because I used server-side rendering (SSR) and hydration,
my actual code is a bit more complex (due to also containing a web server),
but the same general idea still applies.

After that, I decided to start thinking about the content.
More specifically, how would I write the blog posts?
I ended up going with Markdown and converting it to HTML to be displayed in the browser.
I quite like writing Markdown.
It's not too intrusive, unlike HTML, but it's also pretty flexible,
though unfortunately not nearly as powerful as something like reStructuredText.
Okay, Markdown is great, but how would I get my Markdown files to rendered HTML on the client?

I saw two options:

1. Download each Markdown file from the server and render it on the fly in my client
1. Use Leptos' server-side-rendering to render the Markdown on the server and send it off to the client

At this point, I realized it would be infeasible[^1] to go with one of my original ideas
to build a static site generator, which sorta eliminated the first option.
I felt my (not so) skill set wasn't quite up to the task yet.
Maybe I'll come back to it later.
The second option seemed cooler anyway, so I took it and started running.

Leptos' SSR support was a joy to work with.
It was relatively easy to set up a few server-side routes to load and render the posts,
and it all integrated seamlessly with the client.

Here's what a simple server-side function looks like.
This one loads the posts and returns their metadata as a Vec.

```rust
#[server(GetPostMetadata, "/api")]
pub async fn get_post_metadata() -> Result<...> {
    // loads the posts...
    match load_posts().await {
        // ...and sends them off
        Ok(p) => Ok(p),
        // ...or cries in pain
        Err(e) => Err(...),
    }
}
```

Integrating this endpoint into the client is also pretty simple:

```rust
#[component]
pub fn Blog(cx: Scope) -> impl IntoView {
    // asynchronously fetches the posts
    // this returns a future that will contain
    // the posts when the HTTP request is completed
    let posts = create_resource(
        cx,
        || (),  // not reactive
        |_| async { get_post_metadata().await }
    );

    // formats the posts into a view
    let posts_view = move || {
        posts.with(cx, |posts| {
            ...  // snipped post rendering code
            })
        })
    };

    view! { cx,

        <h1>"Blog"</h1>

        // creates a "Suspense", which shows a loading
        // screen while the posts are being fetched
        <Suspense fallback=move || view! {
            cx,
            <p>"Loading posts..."</p>
        }>
            <ul class="posts-list">{posts_view}</ul>
        </Suspense>
    }
}
```

The above code is isomorphic,
meaning it works both on the server and client via SSR and hydration respectively!

I'll admit: I don't fully understand what's going on behind-the-scenes here,
but that's part of the beauty of Leptos!
I'd love to take a deeper dive into its inner workings,
but I'll save that adventure for another time.

Back to Markdown, I used a nice crate called [`Comrak`][comrak] to render the Markdown into HTML.
It supports GitHub-flavored Markdown, which I'd become quite accustomed to after
writing a handful of GitHub READMEs and Jekyll posts.
I needed some way to handle metadata though, and I liked the way Jekyll handled it at the top of the file.

A Jekyll post looks something like this:

```markdown
---
layout: post
title:  "My post"
date:   2023-03-15 00:00:00 +0000
author: Fyssion
permalink: my-post
---

## My post!

...
```

Turns out this is called front matter, and it was pretty easy to fish out
from my Markdown files with a little help from
[Kaylynn's website][kaylynn].
Thanks a bunch!

I ended up using TOML for my front matter because I like the syntax.

At this point, I just needed to put on some finishing touches (styling, error pages, and the like).
I took some inspiration from [PaperMod][papermod] and [Skip's website][skip].
If Skip is somehow reading this, sorry for stealing your domain idea. It was too good!

I absolutely (totally, utterly, etc) suck with colors and design,
so I used [Coolors][coolors] to help me pick some that I liked.
I even asked some friends for their opinions on the site, though I mostly ignored them (sorry!).
Hopefully everything looks okay!

To wrap things up, I just needed a bit of content.
That "Hello World" post just wasn't quite cutting it,
and I wasn't sure what to write about, so I wrote about writing this blog.

And that's it, at least for now.
I'll try to make a habit out of writing these posts,
but no promises!
Thanks for reading.

\- Caleb

[^1]: I think building a static site generator would still feasibly be possible if
    I disabled hydration and pre-rendered every possible route to an HTML file
    (think of frozen-flask).
    I think I'd be able to do it, but to be honest,
    I'm pretty happy with what I've got right now.

[leptos]: https://github.com/leptos-rs/leptos
[leptos-starter]: https://github.com/leptos-rs/start
[trunk]: https://trunkrs.dev/
[comrak]: https://crates.io/crates/comrak
[kaylynn]: https://github.com/kaylynn234/website
[papermod]: https://github.com/adityatelange/hugo-PaperMod
[skip]: https://slice.zone
[coolors]: https://coolors.co

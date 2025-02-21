use leptos::prelude::*;

use crate::utils::post::PostMetadata;

// english reading speed characters per minute
// taken from Firefox's reader mode
// https://searchfox.org/mozilla-central/rev/3da086bd7bce12353fc65968802445dca46f4537/toolkit/components/reader/ReaderMode.sys.mjs#495
static READING_SPEED_CPM: usize = 987;
static READING_SPEED_VARIANCE: usize = 118;

#[component]
pub fn PostMeta(metadata: PostMetadata) -> impl IntoView {
    view! {
        <h3>
            {metadata.created_at.format("%b %e, %Y").to_string()}
            " • "
            {
                // determine word count and separate with commas
                // clever separation solution from https://stackoverflow.com/a/67834588
                metadata.word_count
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(",")
            }
            " words • "
            {
                // estimated reading time adapted from Firefox's reader mode:
                // https://searchfox.org/mozilla-central/rev/3da086bd7bce12353fc65968802445dca46f4537/toolkit/components/reader/ReaderMode.sys.mjs#468-482
                let cpm_low = READING_SPEED_CPM - READING_SPEED_VARIANCE;
                let cpm_high = READING_SPEED_CPM + READING_SPEED_VARIANCE;
                let length = metadata.length;

                let reading_time_slow = length.div_ceil(cpm_low);
                let reading_time_fast = length.div_ceil(cpm_high);

                let formatted = if reading_time_slow == reading_time_fast {
                    reading_time_slow.to_string()
                } else {
                    format!("{}-{}", reading_time_fast, reading_time_slow)
                };

                // lol this doesn't really matter but it's pissing me off
                // just thinking about it being unhandled
                let plural = if reading_time_slow != 1 { "s" } else { "" };
                format!("{} minute{}", formatted, plural)
            }
        </h3>
    }
}

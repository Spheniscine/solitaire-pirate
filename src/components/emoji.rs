use dioxus::{logger::tracing, prelude::*};
use phf::phf_map;

pub static EMOJI_MAP: phf::Map<&'static str, Asset> = phf_map! {
    "🦜" => asset!("/assets/emoji/emoji_u1f99c.svg"),
    "⚓" => asset!("/assets/emoji/emoji_u2693.svg"),
    "🪎" => asset!("/assets/emoji/emoji_u1fa8e.svg"),
    "🦑" => asset!("/assets/emoji/emoji_u1f991.svg"),
    "🐋" => asset!("/assets/emoji/emoji_u1f40b.svg"),
    "🦈" => asset!("/assets/emoji/emoji_u1f988.svg"),
    "😀🏴‍☠️" => asset!("/assets/emoji/moremoji_pirate.svg"),
};

#[component]
pub fn Emoji(text: String) -> Element {
    if let Some(asset) = EMOJI_MAP.get(&text) {
        rsx! {
            img {
                style: "height: 1.15em; vertical-align: middle;",
                src: *asset,
                draggable: false,
                alt: text,
            }
        }
    } else {
        tracing::error!("No emoji asset loaded for string '{text}'");
        rsx! {
            "ERROR"
        }
    }
    
}
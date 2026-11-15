use dioxus::prelude::*;

use crate::{components::{Emoji, SkinTrait}, game::{Card, ColorMode, RANK_FONT_STR, Skin}};

impl SkinTrait<Card> for Skin {
    fn get_color(&self, card: &Card, mode: ColorMode) -> String {
        self.color(*card, mode).to_string()
    }

    fn render_rank(&self, card: &Card) -> Element {
        rsx! {
            span {
                font_family: RANK_FONT_STR,
                {self.rank_text(*card)}
            }
        }
    }

    fn render_suit(&self, card: &Card) -> Element {
        rsx! {
            Emoji {
                text: self.suit_symbol(*card)
            }
        }
    }

    fn render_suit_text(&self, card: &Card) -> Element {
        self.render_suit(card)
    }
}
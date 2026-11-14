use dioxus::prelude::*;

use crate::game::ColorMode;

pub trait SkinTrait<C>: PartialEq + Clone {
    fn get_color(&self, card: &C, mode: ColorMode) -> String;
    fn render_rank(&self, card: &C) -> Element;
    fn render_suit(&self, card: &C) -> Element;
    fn render_suit_text(&self, card: &C) -> Element;
}